#[derive(Clone, Debug, Default)]
pub enum CompressState {
    #[default]
    Ready,
    Compressing,
    Done,
}
