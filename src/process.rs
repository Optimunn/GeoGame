use rand::seq::IteratorRandom;
use serde::Deserialize;
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
pub struct Input;

impl Input {
    #[allow(dead_code)]
    pub fn parse_json(json_str: &str) -> Result<Vec<Country>> {
        let countries = serde_json::from_str(json_str)?;
        Ok(countries)
    }

    pub fn read_from_file(path: &str) -> Result<Vec<Country>> {
        let data = fs::read_to_string(path).unwrap();
        let countries = serde_json::from_str(&data)?;
        Ok(countries)
    }

    pub fn filter_by_continents(countries: Vec<Country>, target_continents: &[Continent],
    ) -> Vec<Country> {
        countries
            .into_iter()
            .filter(|country| match &country.continent {
                Some(cont) => target_continents.contains(cont),
                None => false, // Игнорируем страны без континента
            })
            .collect()
    }

    pub fn get_random_countries(countries: &[Country], count: usize) -> Vec<Country> {
        let mut rng = rand::rng();
        countries
        .iter()
        .choose_multiple(&mut rng, count)
        .into_iter()
        .cloned()
        .collect()
    }
}

use rand::{rngs::ThreadRng, Rng};
use std::{cell::Cell, rc::Rc};
use crate::loadconfig::InputConfig;
pub struct GameLogic;

impl GameLogic {
    pub fn get_image_path(part_of_way_to_image: &String) -> String {
        let patch = format!("assets/{}", part_of_way_to_image);
        patch
    }

    pub fn start_rand_to_image() -> ThreadRng {
        rand::rng()
    }

    pub fn get_rand_to_image(rng: &mut ThreadRng) -> usize {
        let random_number: usize = rng.random_range(0..4);
        random_number
    }

    pub fn get_rand_to_image_cell(rng: &mut ThreadRng) -> Rc<Cell<usize>> {
        let random_number: usize = rng.random_range(0..4);
        Rc::new(Cell::new(random_number))
    }

    pub fn create_continents_list(input_config: &InputConfig) -> Result<Vec<Continent>> {
        use Continent::*;
        const CONTINENTS: [Continent; 6] = [Europe, Asia, Africa, NorthAmerica, SouthAmerica, Oceania];

        let mut out = Vec::new();
        
        for i in 0..CONTINENTS.len() {
            if input_config.continents[i] {
                out.push(CONTINENTS[i].clone());
            }
        }
        Ok(out)
    }

}
