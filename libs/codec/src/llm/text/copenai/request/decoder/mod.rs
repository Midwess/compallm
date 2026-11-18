use super::component::CompatibleOpenAIRequestComponent as Component;
use crate::decoder::{Decoder, DecoderError, DecoderStatus};
use crate::json::{JsonDecodeError, JsonDecoder};
use crate::llm::text::component::TextRequestComponent;
use serde::Deserialize;

const FIELDS: &[&str] = &[
    "model",
    "messages",
    "tools",
    "tool_choice",
    "parallel_tool_calls",
    "max_completion_tokens",
    "top_p",
    "top_k",
    "stop",
    "response_format",
    "provider",
    "stream",
    "stream_options",
];

#[derive(Debug)]
pub struct CompatibleOpenAIRequestDecoder {
    json: JsonDecoder,
    fields: u16,
    field: &'static str,
    has_message: bool,
    finished: bool,
    failed: bool,
}

impl CompatibleOpenAIRequestDecoder {
    pub fn new(json: JsonDecoder) -> Self {
        Self {
            json,
            fields: 0,
            field: "",
            has_message: false,
            finished: false,
            failed: false,
        }
    }
}

impl Decoder for CompatibleOpenAIRequestDecoder {
    type Component<'a> = TextRequestComponent;
    type Error = DecoderError;

    fn next<'a, 'input: 'a>(
        &'a mut self,
        input: &mut &'input [u8],
        eof: bool,
    ) -> Result<DecoderStatus<Self::Component<'a>>, Self::Error> {
        if self.failed || self.finished {
            return Err(DecoderError::InvalidState);
        }
        let result = (|| loop {
            let mut selection_error = None;
            let status = self.json.next_member(input, eof, |key| {
                let key = if key == "max_tokens" {
                    "max_completion_tokens"
                } else {
                    key
                };
                let result = FIELDS
                    .iter()
                    .position(|field| *field == key)
                    .ok_or(DecoderError::Unsupported("request field"))
                    .and_then(|index| {
                        let bit = 1_u16 << index;
                        if self.fields & bit != 0 {
                            return Err(DecoderError::InvalidInput("duplicate request field"));
                        }
                        self.fields |= bit;
                        let field = FIELDS[index];
                        self.field = if field == "messages" {
                            "message"
                        } else {
                            field
                        };
                        Ok(field == "messages")
                    });
                result.map_err(|error| {
                    selection_error = Some(error);
                    JsonDecodeError::InvalidToken
                })
            });
            let status = status.map_err(|_| {
                selection_error.unwrap_or(DecoderError::InvalidInput("invalid request JSON"))
            })?;
            match status {
                DecoderStatus::Component(value) => {
                    if self.field == "top_p" && value == b"null" {
                        continue;
                    }
                    let value: &serde_json::value::RawValue = serde_json::from_slice(value)
                        .map_err(|_| DecoderError::InvalidInput("invalid request value"))?;
                    let deserializer =
                        serde::de::value::MapDeserializer::<_, serde_json::Error>::new(
                            std::iter::once((self.field, value)),
                        );
                    let component = Component::deserialize(
                        serde::de::value::MapAccessDeserializer::new(deserializer),
                    )
                    .map_err(|_| DecoderError::InvalidInput("invalid request component"))?
                    .validate()?;
                    self.has_message |= matches!(component, Component::Message(_));
                    return Ok(DecoderStatus::Component(component.into()));
                }
                DecoderStatus::NeedInput => return Ok(DecoderStatus::NeedInput),
                DecoderStatus::Finished => {
                    if self.fields & 3 != 3 || !self.has_message {
                        return Err(DecoderError::InvalidInput(
                            "model and messages are required",
                        ));
                    }
                    self.finished = true;
                    return Ok(DecoderStatus::Finished);
                }
            }
        })();
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn finish(self) -> Result<(), Self::Error> {
        if self.failed {
            return Err(DecoderError::InvalidState);
        }
        if !self.finished {
            return Err(DecoderError::IncompleteInput);
        }
        self.json
            .finish()
            .map_err(|_| DecoderError::IncompleteInput)
    }
}
