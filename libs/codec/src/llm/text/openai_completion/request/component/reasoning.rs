use crate::llm::text::component::ReasoningEffort;
pub(in crate::llm::text::openai_completion::request) struct EffortWire;
impl EffortWire {
    pub(in crate::llm::text::openai_completion::request) fn parse(
        value: &str,
    ) -> Option<ReasoningEffort> {
        Some(match value {
            "minimal" => ReasoningEffort::Minimal,
            "low" => ReasoningEffort::Low,
            "medium" => ReasoningEffort::Medium,
            "high" => ReasoningEffort::High,
            _ => return None,
        })
    }
    pub(in crate::llm::text::openai_completion::request) fn name(
        value: ReasoningEffort,
    ) -> Option<&'static str> {
        Some(match value {
            ReasoningEffort::Minimal => "minimal",
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
            _ => return None,
        })
    }
}
