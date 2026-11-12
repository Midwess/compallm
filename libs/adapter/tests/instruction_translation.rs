use adapter::{
    codec::TranslateStep,
    error::TranslationError,
    t2t::{
        Text2TextRequestTranslator,
        anthropic::request::{AnthropicRequestDecoder, AnthropicRequestEncoder},
        copenai::request::{CompatibleOpenAIRequestDecoder, CompatibleOpenAIRequestEncoder},
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
    decoder: impl RequestDecoder,
    encoder: impl RequestEncoder,
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

#[tokio::test]
async fn instruction_roles_and_system_blocks_stream_across_protocols() {
    let source = r#"{"model":"source","messages":[
        {"role":"system","content":"Global"}, {"role":"developer","content":"Brief"},
        {"role":"user","content":"One"}, {"role":"developer","content":"No"},
        {"role":"assistant","content":"Cannot"},
        {"role":"user","content":"Two"}, {"role":"developer","content":"Allowed"}
    ]}"#;
    for buffer in [1, 7, 4096] {
        let decoder = OpenAICompletionRequestDecoder::new;
        let chat = translate(
            source,
            decoder(),
            OpenAICompletionRequestEncoder::new("target", Default::default()).unwrap(),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(chat["messages"][1]["role"], "developer");
        assert_eq!(
            chat["messages"][4],
            json!({"role":"assistant","content":"Cannot"})
        );
        let responses = translate(
            source,
            decoder(),
            OpenAIResponseRequestEncoder::new("target", Default::default()).unwrap(),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(responses["input"][3]["role"], "developer");
        let anthropic = translate(
            source,
            decoder(),
            AnthropicRequestEncoder::new("target", Default::default())
                .unwrap()
                .with_mid_conversation_system(true),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(
            anthropic["system"],
            json!([{"type":"text","text":"Global"},{"type":"text","text":"Brief"}])
        );
        assert_eq!(
            anthropic["messages"][1],
            json!({"role":"system","content":"No"})
        );
        assert_eq!(
            anthropic["messages"][4],
            json!({"role":"system","content":"Allowed"})
        );
        let compatible_source = source.replace(
            r#""role":"assistant","content":"Cannot""#,
            r#""role":"assistant","reasoning_content":"source reasoning","content":"Cannot""#,
        );
        let direct = translate(
            &compatible_source,
            CompatibleOpenAIRequestDecoder::new(),
            AnthropicRequestEncoder::new("target", Default::default())
                .unwrap()
                .with_mid_conversation_system(true),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(direct, anthropic);
        let direct_chat = translate(
            &compatible_source,
            CompatibleOpenAIRequestDecoder::new(),
            OpenAICompletionRequestEncoder::new("target", Default::default()).unwrap(),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(direct_chat, chat);
        assert!(
            translate(
                source,
                decoder(),
                AnthropicRequestEncoder::new("target", Default::default()).unwrap(),
                buffer
            )
            .await
            .is_err()
        );
    }
    let blocks = r#"{"model":"source","system":[{"text":"héllo 🌲","type":"text"},{"type":"text","text":"Second"}],"messages":[{"role":"user","content":"hi"}],"max_tokens":73}"#;
    let chat = translate(
        blocks,
        AnthropicRequestDecoder::new(),
        OpenAICompletionRequestEncoder::new("target", Default::default()).unwrap(),
        1,
    )
    .await
    .unwrap();
    assert_eq!(chat["messages"][0]["content"], "héllo 🌲");
    assert_eq!(chat["messages"][1]["content"], "Second");
    let instructions = r#"{"model":"source","instructions":"Be brief","input":"hi"}"#;
    let chat = translate(
        instructions,
        OpenAIResponseRequestDecoder::new(),
        CompatibleOpenAIRequestEncoder::new("target", Default::default()).unwrap(),
        1,
    )
    .await
    .unwrap();
    assert_eq!(
        chat["messages"][0],
        json!({"role":"system","content":"Be brief"})
    );
    let late_role = r#"{"model":"source","messages":[{"content":"héllo","role":"developer"},{"content":"hi","role":"user"}]}"#;
    let settings = source.replace(r#""model":"source""#, r#""model":"source","max_completion_tokens":73,"top_p":0.85,"stream":true,"stream_options":{"include_usage":true}"#);
    let direct = translate(
        &settings,
        CompatibleOpenAIRequestDecoder::new(),
        OpenAICompletionRequestEncoder::new("target", Default::default()).unwrap(),
        1,
    )
    .await
    .unwrap();
    assert_eq!(direct["max_completion_tokens"], 73);
    assert_eq!(direct["top_p"], 0.85);
    assert_eq!(direct["stream"], true);
    assert_eq!(direct["stream_options"]["include_usage"], true);
    let projected = translate(
        late_role,
        OpenAICompletionRequestDecoder::new(),
        OpenAICompletionRequestEncoder::new("target", Default::default()).unwrap(),
        32,
    )
    .await
    .unwrap();
    assert_eq!(
        projected["messages"][0],
        json!({"role":"developer","content":"héllo"})
    );
    let native_mid = r#"{"model":"source","max_tokens":10,"messages":[{"role":"user","content":"u"},{"role":"system","content":"s"},{"role":"assistant","content":"a"},{"role":"user","content":"u2"}]}"#;
    let projected = translate(
        native_mid,
        AnthropicRequestDecoder::new(),
        OpenAIResponseRequestEncoder::new("target", Default::default()).unwrap(),
        1,
    )
    .await
    .unwrap();
    assert_eq!(projected["input"][1]["role"], "system");
    let no_prompt = r#"{"model":"source","messages":[{"role":"user","content":"hi"},{"role":"system","content":"instruction after user"}]}"#;
    let target = translate(
        no_prompt,
        CompatibleOpenAIRequestDecoder::new(),
        AnthropicRequestEncoder::new("target", Default::default())
            .unwrap()
            .with_mid_conversation_system(true),
        1,
    )
    .await
    .unwrap();
    assert_eq!(target["system"], "");
    assert_eq!(
        target["messages"][1],
        json!({"role":"system","content":"instruction after user"})
    );
    let empty_prompt = r#"{"model":"source","messages":[{"role":"system","content":""},{"role":"user","content":"hi"}]}"#;
    let target = translate(
        empty_prompt,
        CompatibleOpenAIRequestDecoder::new(),
        AnthropicRequestEncoder::new("target", Default::default()).unwrap(),
        1,
    )
    .await
    .unwrap();
    assert_eq!(target["system"], "");
    assert_eq!(target["messages"], json!([{"role":"user","content":"hi"}]));
    for messages in [
        json!([{"role":"user","content":"u"},{"role":"system","content":"s"},{"role":"user","content":"u"}]),
        json!([{"role":"user","content":"u"},{"role":"assistant","content":"a"},{"role":"system","content":"s"}]),
    ] {
        let body = json!({"model":"source","messages":messages}).to_string();
        assert!(
            translate(
                &body,
                OpenAICompletionRequestDecoder::new(),
                AnthropicRequestEncoder::new("target", Default::default())
                    .unwrap()
                    .with_mid_conversation_system(true),
                1
            )
            .await
            .is_err()
        );
    }
}

#[tokio::test]
async fn encrypted_responses_reasoning_is_preserved_only_for_explicitly_compatible_targets() {
    let encrypted = "opaque-雪-".repeat(5000);
    let reasoning =
        json!({"type":"reasoning","id":"rs_previous","summary":[],"encrypted_content":encrypted});
    // Keep the type discriminator first while the opaque payload streams.
    let body = format!(
        r#"{{"model":"source","store":false,"include":["reasoning.encrypted_content"],"input":[{{"role":"user","content":"hi"}},{{"type":"reasoning","id":"rs_previous","summary":[],"encrypted_content":{}}},{{"role":"assistant","content":"answer"}},{{"role":"user","content":"next"}}]}}"#,
        serde_json::to_string(&encrypted).unwrap()
    );
    for buffer in [1, 31, 4096] {
        let preserved = translate(
            &body,
            OpenAIResponseRequestDecoder::new(),
            OpenAIResponseRequestEncoder::new("target", Default::default())
                .unwrap()
                .with_compatible_reasoning(true),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(preserved["input"][1], reasoning);
        assert_eq!(preserved["store"], false);
        assert_eq!(preserved["include"], json!(["reasoning.encrypted_content"]));
        let omitted = translate(
            &body,
            OpenAIResponseRequestDecoder::new(),
            OpenAIResponseRequestEncoder::new("target", Default::default()).unwrap(),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(omitted["input"].as_array().unwrap().len(), 3);
        let chat = translate(
            &body,
            OpenAIResponseRequestDecoder::new(),
            CompatibleOpenAIRequestEncoder::new("target", Default::default()).unwrap(),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(
            chat["messages"][1],
            json!({"role":"assistant","content":"answer"})
        );
        let anthropic = translate(
            &body,
            OpenAIResponseRequestDecoder::new(),
            AnthropicRequestEncoder::new("target", Default::default()).unwrap(),
            buffer,
        )
        .await
        .unwrap();
        assert_eq!(anthropic["messages"].as_array().unwrap().len(), 3);
    }
    let id_first = body.replace(
        r#"{"type":"reasoning","id":"rs_previous""#,
        r#"{"id":"rs_previous","type":"reasoning""#,
    );
    let preserved = translate(
        &id_first,
        OpenAIResponseRequestDecoder::new(),
        OpenAIResponseRequestEncoder::new("target", Default::default())
            .unwrap()
            .with_compatible_reasoning(true),
        1,
    )
    .await
    .unwrap();
    assert_eq!(preserved["input"][1], reasoning);
}
