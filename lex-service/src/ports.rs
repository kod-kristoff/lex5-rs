use std::collections::HashMap;

use crate::models::mode::ListAllModesError;

/// `LexService` is the public API for the blog domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait LexService: Clone + Send + Sync + 'static {
    /// Asynchronously list all [Mode]s.
    ///
    /// # Errors
    ///
    /// - [CreateAuthorError::Duplicate] if an [Author] with the same [AuthorName] already exists.
    fn all_modes(
        &self,
    ) -> impl Future<Output = Result<HashMap<String,String>, ListAllModesError>> + Send;
}
