use crate::models::mode::{ListAllModesError, Mode};

/// `LexService` is the public API for the blog domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait LexService: Clone + Send + Sync + 'static {
    /// Asynchronously list all [Mode]s.
    fn all_modes(&self) -> impl Future<Output = Result<Vec<Mode>, ListAllModesError>> + Send;
}

/// `ModeRepository` represents a store of mode data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait ModeRepository: Send + Sync + Clone + 'static {
    /// Asynchronously list all [Mode]s.
    fn all_modes(&self) -> impl Future<Output = Result<Vec<Mode>, ListAllModesError>> + Send;
}
