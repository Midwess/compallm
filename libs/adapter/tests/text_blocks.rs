use adapter::{
    codec::TranslateStep,
    error::TranslationError,
    t2t::{
        Text2TextRequestTranslator,
        anthropic::request::{AnthropicRequestDecoder, AnthropicRequestEncoder},
        copenai::{CompatibleOpenAIRequestDecoder, CompatibleOpenAIRequestEncoder},
        openai::request::{
            OpenAICompletionRequestDecoder, OpenAICompletionRequestEncoder,
            OpenAIResponseRequestDecoder, OpenAIResponseRequestEncoder,
        },
        request::{decoder::RequestDecoder, encoder::RequestEncoder},
    },
};
use futures_util::io::Cursor;
use serde_json::{Value, json};

async fn translate(
    body: &str,
    decoder: Box<dyn RequestDecoder>,
    encoder: Box<dyn RequestEncoder>,
    buffer: usize,
) -> Result<Value, TranslationError> {
    let mut output = Cursor::new(Vec::new());
    let mut translator = Text2TextRequestTranslator::new(buffer, 1);
    translator
        .set_source_stream(Cursor::new(body.as_bytes()))?
        .set_target_stream(&mut output)?
        .set_decoder(decoder)?
        .set_encoder(encoder)?;
    loop {
        match translator.translate()? {
            TranslateStep::NeedInput => {
                translator.next().await?;
            }
            TranslateStep::OutputReady(_) => translator.flush().await?,
            TranslateStep::Progress => tokio::task::yield_now().await,
            TranslateStep::Finished => {
                translator.flush().await?;
                break;
            }
        }
    }
    drop(translator);
    Ok(serde_json::from_slice(output.get_ref()).unwrap())
}

fn decoder(api: usize) -> Box<dyn RequestDecoder> {
    match api {
        0 => Box::new(OpenAICompletionRequestDecoder::new()),
        1 => Box::new(OpenAIResponseRequestDecoder::new()),
        2 => Box::new(AnthropicRequestDecoder::new()),
        _ => Box::new(CompatibleOpenAIRequestDecoder::new()),
    }
}
fn encoder(api: usize) -> Box<dyn RequestEncoder> {
    match api {
        0 => Box::new(OpenAICompletionRequestEncoder::new("target", Default::default()).unwrap()),
        1 => Box::new(OpenAIResponseRequestEncoder::new("target", Default::default()).unwrap()),
        2 => Box::new(AnthropicRequestEncoder::new("target", Default::default()).unwrap()),
        _ => Box::new(CompatibleOpenAIRequestEncoder::new("target", Default::default()).unwrap()),
    }
}

#[tokio::test]
async fn native_text_blocks_preserve_parts_roles_and_bytes_across_targets() {
    let texts = ["héllo 🌲 \"quote\"\\", "separate\npart"];
    for source in 0..4 {
        let mut messages = Vec::new();
        for role in ["user", "assistant"] {
            let parts: Vec<_> = texts
                .iter()
                .map(|text| {
                    if source == 1 && role == "assistant" {
                        json!({"type":"output_text", "text":text, "annotations":[]})
                    } else {
                        json!({"type": if source == 1 {"input_text"} else {"text"}, "text":text})
                    }
                })
                .collect();
            let content = serde_json::to_string(&parts).unwrap();
            let metadata = if source == 1 && role == "assistant" {
                r#", "id":"msg_previous","status":"completed","type":"message""#
            } else {
                ""
            };
            messages.push(format!(
                r#"{{"role":"{role}"{metadata},"content":{content}}}"#
            ));
        }
        let body = format!(
            r#"{{"model":"source","max_tokens":73,"messages":[{}]}}"#,
            messages.join(",")
        );
        let body = if source == 1 {
            body.replace("max_tokens", "max_output_tokens")
                .replace("messages", "input")
        } else {
            body
        };
        for target in 0..4 {
            if source == 3 && target == 1 {
                continue;
            }
            for buffer in [1, 7, 4096] {
                let value = translate(&body, decoder(source), encoder(target), buffer)
                    .await
                    .unwrap();
                let messages = &value[if target == 1 { "input" } else { "messages" }];
                for (index, role) in ["user", "assistant"].iter().enumerate() {
                    assert_eq!(messages[index]["role"], *role);
                    for (part, text) in texts.iter().enumerate() {
                        assert_eq!(messages[index]["content"][part]["text"], *text);
                        assert_eq!(
                            messages[index]["content"][part]["type"],
                            if target == 1 { "input_text" } else { "text" }
                        );
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn long_and_instruction_content_arrays_stream_without_merging_blocks() {
    let long = "large 雪 ".repeat(8192);
    let body = format!(
        r#"{{"model":"source","input":[{{"role":"developer","content":[{{"text":{},"type":"input_text"}},{{"type":"input_text","text":"Second instruction"}}]}},{{"role":"user","content":"hi"}}]}}"#,
        serde_json::to_string(&long).unwrap()
    );
    let value = translate(&body, decoder(1), encoder(2), 7).await.unwrap();
    assert_eq!(
        value["system"],
        json!([{"type":"text","text":long},{"type":"text","text":"Second instruction"}])
    );
    assert_eq!(value["messages"], json!([{"role":"user","content":"hi"}]));
    let mixed = r#"{"model":"source","max_tokens":73,"messages":[{"role":"user","content":[{"text":"one","type":"text"},{"type":"text","text":"two"}]},{"role":"assistant","content":"three"}]}"#;
    let value = translate(mixed, decoder(2), encoder(1), 1).await.unwrap();
    assert_eq!(value["input"][0]["content"].as_array().unwrap().len(), 2);
    assert_eq!(value["input"][1]["content"], "three");
}
