use slint::Color;
use rand::seq::IteratorRandom;
use rand::Rng;

use crate::configure::{Country, Continent};
use crate::threadfn::GameMode;
use crate::consts::pallet::*;

pub struct GameLogic;

impl GameLogic {

    pub fn filter_by_continents(countries: &Vec<Country>, target_continents: &[Continent]
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

    pub fn get_rand_universal(count: usize) -> usize {
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let random_number: usize = rng.random_range(0..count);
        random_number
    }

    pub fn create_continents_list(input_config: &Vec<bool>) -> Vec<Continent> {
        use Continent::*;
        const CONTINENTS: [Continent; 6] = [Europe, Asia, Africa, NorthAmerica, SouthAmerica, Oceania];

        let mut out = Vec::new();

        for i in 0..CONTINENTS.len() {
            if input_config[i] {
                out.push(CONTINENTS[i].clone());
            }
        }
        out
    }

    pub fn create_mode_list(input_config: &Vec<bool>) -> Vec<GameMode> {
        use GameMode::*;
        const MODE: [GameMode; 3] = [Flags, Capitals, Fandc];

        let mut out = Vec::new();

        for i in 0..MODE.len() {
            if input_config[i] {
                out.push(MODE[i].clone());
            }
        }
        out
    }

    pub fn ret_button_color(index: i32) -> Color {
        match index {
            0 => COLOR_GRAY,
            1 => COLOR_FREEDOM,
            2 => COLOR_LAVENDER,
            3 => COLOR_BLUE_SKY,
            4 => COLOR_MANDARIN,
            5 => COLOR_RIPE_LIME,
            _ => COLOR_GRAY
        }
    }

    pub fn ret_button_color_index(name: &str) -> i32 {
        match name {
            "gray" => 0,
            "freedom" => 1,
            "lavender" => 2,
            "blue_sky" => 3,
            "mandarin" => 4,
            "ripe_lime" => 5,
            _ => 0
        }
    }

    pub fn ret_button_color_string(index: i32) -> String {
        match index {
            0 => "gray",
            1 => "freedom",
            2 => "lavender",
            3 => "blue_sky",
            4 => "mandarin",
            5 => "ripe_lime",
            _ => "gray"
        }.to_string()
    }
}