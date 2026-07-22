use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("cannot determine user home directory")]
    NoHomeDir,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub default_theme: String,
    #[serde(default = "default_add_headers")]
    pub add_headers: bool,
    #[serde(default)]
    pub always_dry_run_first: bool,
    #[serde(default = "default_use_nerd_fonts")]
    pub use_nerd_fonts: bool,
}

fn default_add_headers() -> bool {
    true
}

fn default_use_nerd_fonts() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_theme: "obsidian".to_string(),
            add_headers: true,
            always_dry_run_first: false,
            use_nerd_fonts: true,
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir().ok_or(ConfigError::NoHomeDir)?;
        Ok(config_dir.join("gitnibble").join("config.toml"))
    }

    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path()?;
        if !path.exists() {
            let cfg = Config::default();
            let _ = Self::save(&cfg);
            return Ok(cfg);
        }
        let raw = fs::read_to_string(&path)?;
        Ok(toml::from_str(&raw)?)
    }

    pub fn save(cfg: &Config) -> Result<(), ConfigError> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let raw = toml::to_string_pretty(cfg)?;
        fs::write(&path, raw)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_roundtrip() {
        let cfg = Config::default();
        let serialized = toml::to_string_pretty(&cfg).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(cfg.default_theme, deserialized.default_theme);
        assert_eq!(cfg.add_headers, deserialized.add_headers);
        assert_eq!(cfg.always_dry_run_first, deserialized.always_dry_run_first);
        assert_eq!(cfg.use_nerd_fonts, deserialized.use_nerd_fonts);
    }
}
