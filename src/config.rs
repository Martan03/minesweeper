use std::{
    fs::File,
    io::{BufReader, BufWriter},
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
        Self::from_json(config_file()).unwrap_or_default()
    }

    pub fn from_json(path: impl AsRef<Path>) -> Result<Self> {
        let f = BufReader::new(File::open(path)?);
        Ok(serde_json::from_reader(f)?)
    }

    pub fn to_default_json(&self) -> Result<()> {
        self.to_json(config_file())
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> Result<()> {
        let f = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| ".".into())
        .join("minesweeper")
}

pub fn config_file() -> PathBuf {
    config_dir().join("config.json")
}
