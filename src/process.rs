pub mod gamelogic {

    use slint::Color;
    use rand::seq::IteratorRandom;
    use rand::Rng;

    use crate::configure::{Country, Continent};
    use crate::consts::ui;
    use crate::threadfn::GameMode;

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
        use crate::consts::pallet::*;
        match index {
            ui::C_GRAY => GRAY,
            ui::C_FREEDOM => FREEDOM,
            ui::C_LAVENDER => LAVENDER,
            ui::C_BLUE_SKY => BLUE_SKY,
            ui::C_MANDARIN => MANDARIN,
            ui::C_RIPE_LIME => RIPE_LIME,
            _ => GRAY
        }
    }

    pub fn ret_button_color_index(name: &str) -> i32 {
        match name {
            "gray" => ui::C_GRAY,
            "freedom" => ui::C_FREEDOM,
            "lavender" => ui::C_LAVENDER,
            "blue_sky" => ui::C_BLUE_SKY,
            "mandarin" => ui::C_MANDARIN,
            "ripe_lime" => ui::C_RIPE_LIME,
            _ => ui::C_GRAY
        }
    }

    pub fn ret_button_color_string(index: i32) -> String {
        match index {
            ui::C_GRAY => "gray",
            ui::C_FREEDOM => "freedom",
            ui::C_LAVENDER => "lavender",
            ui::C_BLUE_SKY => "blue_sky",
            ui::C_MANDARIN => "mandarin",
            ui::C_RIPE_LIME => "ripe_lime",
            _ => "gray"
        }.to_string()
    }

    pub fn ret_language_index(name: &str) -> i32 {
        match name {
            "en" => ui::I_EN,
            "ru" => ui::I_RU,
            _ => ui::I_EN
        }
    }

    pub fn ret_language_string(index: i32) -> String {
        match index {
            ui::I_EN => "en",
            ui::I_RU => "ru",
            _ => "en"
        }.to_string()
    }
}