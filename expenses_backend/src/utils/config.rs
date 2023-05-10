extern crate toml;

use serde::Deserialize;
use std::io::Read;
use std::{fs, path};

use crate::database;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(deserialize_with = "parse_log_filter")]
    log_level: log::LevelFilter,
    db_config: database::DbConfig,
}

impl Config {
    pub fn log_level(&self) -> log::LevelFilter {
        self.log_level
    }

    pub fn db_config(&self) -> &database::DbConfig {
        &self.db_config
    }
}

pub fn read_config<P: AsRef<path::Path>, T: for<'de> Deserialize<'de>>(
    path: P,
) -> Result<T, String> {
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    toml::from_str(&contents).map_err(|e| e.to_string())
}

fn parse_log_filter<'de, D>(deserializer: D) -> Result<log::LevelFilter, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "trace" => Ok(log::LevelFilter::Trace),
        "debug" => Ok(log::LevelFilter::Debug),
        "info" => Ok(log::LevelFilter::Info),
        "warn" => Ok(log::LevelFilter::Warn),
        "error" => Ok(log::LevelFilter::Error),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid log level: {}",
            s
        ))),
    }
}
