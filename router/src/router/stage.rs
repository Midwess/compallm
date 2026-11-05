pub use sec::stage::*;

pub struct HeaderReceived;
impl StageLabel for HeaderReceived {}

/// Runs after HeaderReceived succeeds, before consuming body bytes.
pub struct BodyStarted;
impl StageLabel for BodyStarted {}
