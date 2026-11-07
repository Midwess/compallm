use std::num::NonZeroUsize;

/// Driver bounds, not a bound on transport allocations or codec-retained data.
#[derive(Clone, Copy, Debug)]
pub struct TranslationLimits {
    pub output_chunk_bytes: NonZeroUsize,
    /// Maximum driver iterations per poll. Codec calls must also do bounded work.
    pub steps_per_poll: NonZeroUsize,
}

impl Default for TranslationLimits {
    fn default() -> Self {
        Self {
            output_chunk_bytes: NonZeroUsize::new(16 * 1024).unwrap(),
            steps_per_poll: NonZeroUsize::new(64).unwrap(),
        }
    }
}
