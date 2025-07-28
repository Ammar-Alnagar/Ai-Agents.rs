use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub agents: Vec<AgentConfig>,
    pub providers: ProvidersConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentConfig {
    pub name: String,
    pub tools: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvidersConfig {
    pub openai_api_key: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str).unwrap();
        Ok(config)
    }
}
