/*!
   Module `service` provides the canonical implementation of the [LexService] port. All
   lex-domain logic is defined here.
*/

use crate::{
    models::mode::{FieldMappings, ListAllModesError, Mode, ModeFieldmappingsError},
    ports::{LexService, ModeRepository},
};

/// Canonical implementation of the [LexService] port, through which the blog domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<MR>
where
    MR: ModeRepository,
{
    mode_repo: MR,
}

impl<MR> Service<MR>
where
    MR: ModeRepository,
{
    pub fn new(mode_repo: MR) -> Self {
        Self { mode_repo }
    }
}
impl<MR> LexService for Service<MR>
where
    MR: ModeRepository,
{
    async fn all_modes(&self) -> Result<Vec<Mode>, ListAllModesError> {
        self.mode_repo.all_modes().await
    }

    async fn get_fieldmappings(
        &self,
        mode: &str,
    ) -> Result<Option<FieldMappings>, ModeFieldmappingsError> {
        self.mode_repo.get_fieldmappings(mode).await
    }
}
