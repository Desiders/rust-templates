#[derive(Debug, thiserror::Error)]
#[allow(unused)]
pub enum ErrKind<E> {
    #[error(transparent)]
    Expected(E),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
