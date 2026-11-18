use crate::decoder::DecoderError;
use crate::json::JsonToken;
use crate::llm::text::component::{ReasoningDetail, ReasoningDetailContent};

#[derive(Debug, Default)]
pub(super) struct ReasoningState {
    pub(super) active: bool,
    details: Vec<ReasoningDetail>,
    current: Option<DetailDraft>,
}

#[derive(Debug)]
enum Kind {
    Summary,
    Text,
    Encrypted,
}

#[derive(Debug, Default)]
struct DetailDraft {
    kind: Option<Kind>,
    summary: Option<String>,
    text: Option<String>,
    data: Option<String>,
    signature: Option<Option<String>>,
    id: Option<String>,
    format: Option<String>,
    index: Option<u32>,
}

impl ReasoningState {
    pub(super) fn proceed(
        &mut self,
        token: JsonToken<'_>,
    ) -> Result<Option<Vec<ReasoningDetail>>, DecoderError> {
        if !self.active {
            if !matches!(token, JsonToken::ValueArray(Some("reasoning_details"))) {
                return Err(DecoderError::InvalidInput(
                    "reasoning details must be an array",
                ));
            }
            self.active = true;
            return Ok(None);
        }
        if let Some(detail) = &mut self.current {
            match token {
                JsonToken::ValueString(Some("type"), value) => {
                    detail.kind = Some(match value {
                        "summary" | "reasoning.summary" => Kind::Summary,
                        "text" | "reasoning.text" => Kind::Text,
                        "encrypted" | "reasoning.encrypted" => Kind::Encrypted,
                        _ => return Err(DecoderError::InvalidInput("invalid reasoning type")),
                    })
                }
                JsonToken::ValueString(Some("summary"), value) => {
                    detail.summary = Some(value.to_owned())
                }
                JsonToken::ValueString(Some("text"), value) => detail.text = Some(value.to_owned()),
                JsonToken::ValueString(Some("data"), value) => detail.data = Some(value.to_owned()),
                JsonToken::ValueString(Some("signature"), value) => {
                    detail.signature = Some(Some(value.to_owned()))
                }
                JsonToken::ValueNull(Some("signature")) => detail.signature = Some(None),
                JsonToken::ValueString(Some("id"), value) => detail.id = Some(value.to_owned()),
                JsonToken::ValueString(Some("format"), value) => {
                    detail.format = Some(value.to_owned())
                }
                JsonToken::ValueNumber(Some("index"), value) => {
                    detail.index = Some(
                        value
                            .as_u64()
                            .and_then(|v| u32::try_from(v).ok())
                            .ok_or(DecoderError::InvalidInput("invalid reasoning index"))?,
                    )
                }
                JsonToken::ValueNull(Some("id" | "format" | "index")) => {}
                JsonToken::ObjectEnd => {
                    let detail = self.current.take().ok_or(DecoderError::InvalidState)?;
                    self.details.push(detail.try_into()?);
                }
                _ => return Err(DecoderError::InvalidInput("invalid reasoning field")),
            }
            return Ok(None);
        }
        match token {
            JsonToken::ValueObject(None) => {
                self.current = Some(DetailDraft::default());
                Ok(None)
            }
            JsonToken::ArrayEnd => {
                self.active = false;
                Ok(Some(std::mem::take(&mut self.details)))
            }
            _ => Err(DecoderError::InvalidInput(
                "reasoning detail must be an object",
            )),
        }
    }
}

impl TryFrom<DetailDraft> for ReasoningDetail {
    type Error = DecoderError;
    fn try_from(detail: DetailDraft) -> Result<Self, Self::Error> {
        let invalid = DecoderError::InvalidInput("invalid reasoning detail");
        let content = match detail.kind {
            Some(Kind::Summary)
                if detail.text.is_none() && detail.data.is_none() && detail.signature.is_none() =>
            {
                ReasoningDetailContent::Summary {
                    summary: detail.summary.ok_or(invalid)?,
                }
            }
            Some(Kind::Text) if detail.summary.is_none() && detail.data.is_none() => {
                ReasoningDetailContent::Text {
                    text: detail.text.ok_or(invalid)?,
                    signature: detail.signature.flatten(),
                }
            }
            Some(Kind::Encrypted)
                if detail.summary.is_none()
                    && detail.text.is_none()
                    && detail.signature.is_none() =>
            {
                ReasoningDetailContent::Encrypted {
                    data: detail.data.ok_or(invalid)?,
                }
            }
            _ => return Err(invalid),
        };
        Ok(Self {
            content,
            id: detail.id,
            format: detail.format,
            index: detail.index,
        })
    }
}
