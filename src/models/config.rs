use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub filters: Filters,
    pub automation: Automation,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Filters {
    pub unwanted_keywords: Vec<String>,
    pub relevant_keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Automation {
    pub delay_min_ms: u64,
    pub delay_max_ms: u64,
    pub batch_size: usize,
    pub pause_between_batches_ms: u64,
    pub daily_limit: usize,
    pub dry_run: bool,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
