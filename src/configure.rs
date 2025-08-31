use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Result;
use std::fs;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Continent {
    Africa,
    Asia,
    Europe,
#[serde(rename = "North America")]
    NorthAmerica,
#[serde(rename = "South America")]
    SouthAmerica,
    Oceania,
#[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Country {
#[allow(dead_code)]
    pub capital: Option<String>,
#[allow(dead_code)]
    pub code: String,
    pub continent: Option<Continent>,
#[allow(dead_code)]
    flag_1x1: String,
#[allow(dead_code)]
    pub flag_4x3: String,
#[allow(dead_code)]
    iso: bool,
#[allow(dead_code)]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputConfig {
    pub continents: Vec<bool>
}
pub struct ConfigurationSettings;

impl ConfigurationSettings {

    pub fn read_from_file<T: DeserializeOwned>(path: &str) -> Result<T> {
        let data: String = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(_) => String::from(""),
        };
        let result = serde_json::from_str(&data)?;
        Ok(result)
    }

#[allow(dead_code)]
    pub fn write_input_config(path: &str, input: &InputConfig) -> Result<()> {
        let file: fs::File = fs::File::create(path).unwrap();
        let output = serde_json::to_writer_pretty(file, input)?;
        Ok(output)
    }
}