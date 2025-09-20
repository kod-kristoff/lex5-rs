use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum ListAllModesError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// A mode.
#[derive(Debug, Clone)]
pub struct Mode {
    id: ModeId,
}

impl Mode {
    pub fn new(id: ModeId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &ModeId {
        &self.id
    }
}

#[derive(Debug, Clone)]
/// A valid mode id.
pub struct ModeId(String);

#[derive(Clone, Debug, thiserror::Error)]
#[error("mode id cannot be empty")]
pub struct ModeIdEmptyError;

impl ModeId {
    pub fn new(raw: &str) -> Result<Self, ModeIdEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(ModeIdEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl fmt::Display for ModeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
