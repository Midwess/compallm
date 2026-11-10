/// Shared fallback settings for text-to-text translation.
/// Explicit source settings replace these defaults; this is not a token cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Text2TextTranslatorConfiguration {
    /// Maximum output tokens, mapped to the target protocol's token-limit field.
    /// Must exceed zero. Provider/model-specific limits remain caller-owned.
    pub max_output_tokens: u32,
}

impl Default for Text2TextTranslatorConfiguration {
    fn default() -> Self {
        Self {
            max_output_tokens: 32 * 1024,
        }
    }
}
