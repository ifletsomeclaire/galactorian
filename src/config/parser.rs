use std::error::Error;
use serde::{Deserialize, Serialize};

use crate::errors::GalError;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub fullscreen: bool,
}

pub fn parse_toml() -> Result<Config, GalError> {
    let path = "assets/config.toml";
    let string = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&string)?;
    Ok(config)
}