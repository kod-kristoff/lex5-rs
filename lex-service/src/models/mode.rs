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
    groups: Option<Vec<ModeId>>,
}

impl Mode {
    pub fn new(id: ModeId) -> Self {
        Self { id, groups: None }
    }

    pub fn id(&self) -> &ModeId {
        &self.id
    }

    pub fn groups(&self) -> Option<&Vec<ModeId>> {
        self.groups.as_ref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
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
