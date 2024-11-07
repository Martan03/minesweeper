use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{args::Difficulty, error::Result};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default_difficulty: Option<Difficulty>,
}

impl Config {
    pub fn from_default_json() -> Self {
        Self::from_json(config_path().join("config.json")).unwrap_or_default()
    }

    pub fn from_json(path: impl AsRef<Path>) -> Result<Self> {
        let f = BufReader::new(File::open(path)?);
        Ok(serde_json::from_reader(f)?)
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| ".".into())
        .join("minesweeper")
}
