use rand::seq::IteratorRandom;
use serde_json::Result;
use rand::{rngs::ThreadRng, Rng};
use std::{cell::Cell, rc::Rc};

use crate::configure::{Country, Continent};

pub struct GameLogic;

impl GameLogic {

    pub fn filter_by_continents(countries: &Vec<Country>, target_continents: &[Continent],
    ) -> Vec<Country> {
        countries
            .into_iter()
            .filter(|country| match &country.continent {
                Some(cont) => target_continents.contains(cont),
                None => false,
            })
            .cloned()
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

    pub fn get_image_path(part_of_way_to_image: &String) -> String {
        let patch = format!("assets/{}", part_of_way_to_image);
        patch
    }
#[inline(always)]
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

    pub fn create_continents_list(input_config: &Vec<bool>) -> Result<Vec<Continent>> {
        use Continent::*;
        const CONTINENTS: [Continent; 6] = [Europe, Asia, Africa, NorthAmerica, SouthAmerica, Oceania];

        let mut out = Vec::new();
        
        for i in 0..CONTINENTS.len() {
            if input_config[i] {
                out.push(CONTINENTS[i].clone());
            }
        }
        Ok(out)
    }
}
