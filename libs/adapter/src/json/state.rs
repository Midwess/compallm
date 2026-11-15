#[derive(Clone, Copy, Debug, Default)]
pub(super) enum State {
    #[default]
    Idle,
    String {
        escaped: bool,
    },
    Scalar,
    Finished,
    Failed,
}
