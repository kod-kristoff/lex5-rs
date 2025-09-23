use std::{borrow::Borrow, collections::HashMap, fs, io::BufReader};

use anyhow::Context;
use lex_service::{
    models::mode::{ListAllModesError, Mode, ModeId},
    ports::ModeRepository,
};

#[derive(Debug, Clone)]
pub struct JsonModeRepository {
    modes: HashMap<ModeId, Mode>,
}

impl JsonModeRepository {
    pub fn new(path: &str) -> Result<Self, anyhow::Error> {
        let reader = BufReader::new(
            fs::File::open(path).with_context(|| format!("invalid mode repo path: '{}'", path))?,
        );
        let modes = serde_json::from_reader(reader).with_context(|| "Failed to read JSON")?;
        let modes = set_defaults(modes);
        Ok(Self { modes })
    }
}

impl ModeRepository for JsonModeRepository {
    async fn all_modes(&self) -> Result<Vec<Mode>, ListAllModesError> {
        Ok(self.modes.values().cloned().collect())
    }
}

fn set_defaults(mut modes: HashMap<ModeId, Mode>) -> HashMap<ModeId, Mode> {
    if let Some(defaults) = modes.get("default").cloned() {
        for (data_key, data_val) in modes.iter_mut() {
            if data_key != "default" {
                data_val.update_from_defaults(&defaults);
            }
        }
    }
    modes
}
