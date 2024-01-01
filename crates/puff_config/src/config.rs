use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub vcs: Option<VcsConfig>,
    pub paths: Vec<String>,
    pub ignored_paths: Vec<String>,
    pub formatter: Option<FormatterConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VcsConfig {
    pub system: SupportedVcs,
    pub ignore_file: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SupportedVcs {
    #[serde(rename = "git")]
    Git,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormatterConfig {
    pub rules: RulesConfig,
}

pub type RulesConfig = HashMap<String, HashMap<String, toml::Value>>;
