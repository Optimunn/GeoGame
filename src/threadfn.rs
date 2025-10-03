use slint::{SharedString, ToSharedString};
use std::sync::mpsc::Sender;
use std::fs;
#[cfg(not(debug_assertions))]
use std::path::PathBuf;
#[cfg(debug_assertions)]
use crate::consts::os::LOAD_IMAGE;
use crate::consts::ui;
use crate::process::gamelogic;
use crate::configure::Country;
use crate::null_ss;

#[derive(PartialEq)]
pub enum Action {
    Update,
    Load
}

#[derive(PartialEq, Clone)]
pub enum GameMode {
    Flags,
    Capitals,
    Fandc
}

pub struct CountryData {
    pub name: SharedString,
    pub capital: SharedString,
    pub code: SharedString,
    pub continent: SharedString,
    pub img: Vec<u8>,
}

use crate::slint_generatedMainWindow::Information;
use crate::configure::get;

impl CountryData {
    pub fn default() -> Self {
        CountryData {
            name: null_ss!(),
            capital: null_ss!(),
            code: null_ss!(),
            continent: null_ss!(),
            img: vec![1;1],
        }
    }

    pub fn to_info(self) -> Information {
        Information {
            name: self.name,
            capital: self.capital,
            code: self.code,
            continent: self.continent,
            img: get::img(&self.img)
        }
    }
}

pub struct ThreadData {
    pub mode: GameMode,
    pub data: CountryData,
    pub names: Vec<SharedString>
}

pub struct ThreadIn {
    pub mode: Option<Vec<GameMode>>,
    pub action: Action,
    pub checkbox: Option<Vec<bool>>,
    pub random: Option<usize>
}

use crate::translation::ContinentsTranslation;

#[inline(always)]
pub fn load_data_from_thread(
    filtered_cont: &Vec<Country>,
    mode: &Vec<GameMode>,
    input: &ThreadIn,
    tx_data: &Sender<ThreadData>,
    tr_cont: &ContinentsTranslation,
#[cfg(not(debug_assertions))]
    image_path_string: &PathBuf
) {
    let mut model: Vec<SharedString> = vec![SharedString::new(); ui::ANSWER_NUM];
    let used_countries: Vec<Country> = gamelogic::get_random_countries(&filtered_cont, ui::ANSWER_NUM);
    let used_mode: GameMode = mode[gamelogic::get_rand_universal(mode.len())].clone();
    let mut data_out: CountryData = CountryData::default();

    use GameMode::*;
    match used_mode {
        Flags => {
            for i in 0..ui::ANSWER_NUM { model[i] = used_countries[i].name.to_shared_string(); }
        }
        Capitals => {
            for i in 0..ui::ANSWER_NUM {
                let exit = match &used_countries[i].capital {
                    None => { null_ss!() },
                    Some(capital) => { capital.to_shared_string() }
                };
                model[i] = exit;
            }
        }
        Fandc => {
            println!("TODO: FandC");
        }
    }

    let rand_unwrap: usize = input.random.unwrap();
#[cfg(debug_assertions)]
    let patch: String = used_countries[rand_unwrap].flag_4x3.to_string();
#[cfg(debug_assertions)]
    let patch: String = format!("{LOAD_IMAGE}{}", patch);
#[cfg(not(debug_assertions))]
    let patch: PathBuf = image_path_string.join(used_countries[rand_unwrap].flag_4x3.as_str());
    let image_data: Vec<u8> = match fs::read(patch) {
        Ok(data) => data,
        Err(_) => panic!("Failed to load image")
    };

    data_out.name = used_countries[rand_unwrap].name.to_shared_string();
    data_out.capital = match &used_countries[rand_unwrap].capital {
        Some(capital) => { capital.to_shared_string() },
        None => { null_ss!() }
    };
    data_out.code = used_countries[rand_unwrap].code.to_shared_string();
    data_out.continent = match &used_countries[rand_unwrap].continent {
        Some(continent) => { continent.ret_continent_name(&tr_cont) },
        None => { null_ss!() }
    };
    data_out.img = image_data;
    //data_out.iso = used_countries[rand_unwrap].iso;

    let data: ThreadData = ThreadData {
        mode: used_mode,
        data: data_out,
        names: model
    };
    tx_data.send(data).unwrap();
}