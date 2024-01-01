mod config;

use std::error::Error;

pub use config::*;

pub fn parse_config(content: &str) -> Result<Config, Box<dyn Error>> {
    toml::from_str(content).map_err(|err| err.into())
}
