use rand::seq::IteratorRandom;
use serde_json::Result;
use rand::{rngs::ThreadRng, Rng};

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

#[inline(always)]
    pub fn start_rand_thread() -> ThreadRng {
        rand::rng()
    }

    pub fn get_rand_universal(rng: &mut ThreadRng) -> usize {
        let random_number: usize = rng.random_range(0..4);
        random_number
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
