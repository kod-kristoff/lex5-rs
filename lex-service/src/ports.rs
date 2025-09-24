use crate::models::mode::{FieldMappings, ListAllModesError, Mode, ModeFieldmappingsError};

/// `LexService` is the public API for the blog domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait LexService: Clone + Send + Sync + 'static {
    /// Asynchronously list all [Mode]s.
    fn all_modes(&self) -> impl Future<Output = Result<Vec<Mode>, ListAllModesError>> + Send;
    /// Asynchronously get field mappings for a [Mode].
    fn get_fieldmappings(
        &self,
        mode: &str,
    ) -> impl Future<Output = Result<Option<FieldMappings>, ModeFieldmappingsError>> + Send;
}

/// `ModeRepository` represents a store of mode data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait ModeRepository: Send + Sync + Clone + 'static {
    /// Asynchronously list all [Mode]s.
    fn all_modes(&self) -> impl Future<Output = Result<Vec<Mode>, ListAllModesError>> + Send;
    /// Asynchronously get field mappings for a [Mode].
    fn get_fieldmappings(
        &self,
        mode: &str,
    ) -> impl Future<Output = Result<Option<FieldMappings>, ModeFieldmappingsError>> + Send;
}
