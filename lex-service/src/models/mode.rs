#[derive(Debug, thiserror::Error)]
pub enum ListAllModesError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
