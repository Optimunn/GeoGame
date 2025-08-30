use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputConfig {
    pub continents: Vec<bool>
}
pub struct ConfigurationSettings;

impl ConfigurationSettings {
    
    pub fn read_input_config(path: &str) -> Result<InputConfig> {
        let data = fs::read_to_string(path).unwrap();
        let input = serde_json::from_str(&data)?;
        Ok(input)
    }

#[allow(dead_code)]
    pub fn write_input_config(path: &str, input: &InputConfig) -> Result<()> {
        let file = fs::File::create(path).unwrap();
        let output = serde_json::to_writer_pretty(file, input)?;
        Ok(output)
    }
}