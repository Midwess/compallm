#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reasoning {
    /// Leaves model defaults unchanged when absent.
    pub enabled: Option<bool>,
    /// Selects reasoning strength instead of an explicit reasoning-token budget.
    pub effort: Option<ReasoningEffort>,
    /// Limits reasoning tokens only, separately from the total output-token limit.
    pub max_tokens: Option<u32>,
    /// Hides returned reasoning without disabling its generation.
    pub exclude: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
    Max,
}
