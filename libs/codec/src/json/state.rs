#[derive(Clone, Copy, Debug, Default)]
pub(super) enum JsonState {
    #[default]
    Idle,
    String {
        escaped: bool,
    },
    Scalar,
    Finished,
    Failed,
}
