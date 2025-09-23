use std::{env, path::PathBuf};

use anyhow::Context;

const INSTANCE_PATH_KEY: &str = "KARP5_INSTANCE_PATH";
const SERVER_PORT_KEY: &str = "SERVER_PORT";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub instance_path: PathBuf,
    config_path: PathBuf,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let server_port = load_env(SERVER_PORT_KEY)
            .ok()
            .unwrap_or_else(|| "3000".to_string());
        let instance_path: PathBuf = load_env(INSTANCE_PATH_KEY)
            .ok()
            .unwrap_or_else(|| "./".to_string())
            .into();
        let config_path = instance_path.join("config");
        Ok(Config {
            server_port,
            instance_path,
            config_path,
        })
    }

    pub fn modes_path(&self) -> PathBuf {
        let mut modes_path = self.config_path.clone();
        modes_path.push("modes.json");
        modes_path
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}
