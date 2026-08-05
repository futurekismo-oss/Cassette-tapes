use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
pub struct TapeManifest {
    pub tape: TapeInfo,
    pub dependencies: Option<Dependecies>,
    pub targets: Option<HashMap<String, String>>,
    pub hooks: Option<Hooks>,
}

#[derive(Debug, Deserialize)]
pub struct TapeInfo {
    pub name: String,
    pub version: String,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct Dependecies {
    pub binaries: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Hooks {
    #[serde(default)]
    pub insert: Vec<String>,

    #[serde(default)]
    pub eject: Vec<String>,
}

impl TapeManifest {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read file: {:?}", path.as_ref()))?;

        let manifest: TapeManifest =
            toml::from_str(&content).context("Failed to parse TOML file")?;

        Ok(manifest)
    }
}
