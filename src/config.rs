//! Configuration options

use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use etcetera::{AppStrategy, AppStrategyArgs};
use eyre::Result;
use serde::Deserialize;

use crate::prayertime::Zones;

pub static CONFIG_FILE: LazyLock<PathBuf> =
    LazyLock::new(|| etcetera_def().unwrap().config_dir().join("config.toml"));

pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| etcetera_def().unwrap().cache_dir());

#[derive(Deserialize)]
pub struct Config {
    default_zone: Zones,
}

impl Config {
    pub fn new(path: Option<&Path>) -> eyre::Result<Self> {
        if let Some(path) = path {
            Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
        } else if CONFIG_FILE.exists() {
            Ok(toml::from_str(&std::fs::read_to_string(
                CONFIG_FILE.as_path(),
            )?)?)
        } else {
            Ok(Self {
                default_zone: Zones::SGR01,
            })
        }
    }

    pub fn get_default_zone(&self) -> Zones {
        self.default_zone.clone()
    }
}

fn etcetera_def() -> Result<impl AppStrategy> {
    Ok(etcetera::choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "solemnattic".to_string(),
        app_name: "praytime".to_string(),
    })?)
}
