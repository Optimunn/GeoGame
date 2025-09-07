use slint::{SharedString, ToSharedString};
use std::sync::mpsc::Sender;
use std::fs;
#[cfg(not(debug_assertions))]
use std::path::PathBuf;
#[cfg(debug_assertions)]
use crate::consts::LOAD_IMAGE;
use crate::process::GameLogic;
use crate::configure::Country;

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

pub struct ThreadData {
    pub mode: GameMode,
    pub img: Option<Vec<u8>>,
    pub text: Option<SharedString>,
    pub names: Vec<SharedString>
}

pub struct ThreadIn {
    pub mode: Option<Vec<GameMode>>,
    pub action: Action,
    pub checkbox: Option<Vec<bool>>,
    pub random: Option<usize>
}

#[inline(always)]
pub fn load_data_from_thread(
    filtered_cont: &Vec<Country>,
    mode: &Vec<GameMode>,
    input: &ThreadIn,
    tx_data: &Sender<ThreadData>,
#[cfg(not(debug_assertions))]
    image_path_string: &PathBuf
) {
    let mut model: Vec<SharedString> = vec![SharedString::new(); 4];
    let out4: Vec<Country> = GameLogic::get_random_countries(&filtered_cont, 4);
    let used_mode: GameMode = mode[GameLogic::get_rand_universal(mode.len())].clone();

    use GameMode::*;
    match used_mode {
        Flags => {
        #[cfg(debug_assertions)]
            let patch: String = out4[input.random.unwrap()].flag_4x3.to_string();
        #[cfg(debug_assertions)]
            let patch: String = format!("{LOAD_IMAGE}{}", patch);
        #[cfg(not(debug_assertions))]
            let patch: PathBuf = image_path_string.join(out4[input.random.unwrap()].flag_4x3.as_str());
            let image_data: Vec<u8> = match fs::read(patch) {
                Ok(data) => data,
                Err(_) => panic!("Failed to load image")
            };
            for i in 0..4 { model[i] = out4[i].name.to_shared_string(); }

            let data: ThreadData = ThreadData { mode: used_mode, text: None, img: Some(image_data), names: model };
            tx_data.send(data).unwrap();
        }
        Capitals => {
            let text: SharedString = out4[input.random.unwrap()].name.to_shared_string();

            for i in 0..4 {
                let exit = match &out4[i].capital {
                    None => { "None".to_shared_string() },
                    Some(capital) => { capital.to_shared_string() }
                };
                model[i] = exit;
            }

            let data: ThreadData = ThreadData { mode: used_mode, text: Some(text), img: None, names: model };
            tx_data.send(data).unwrap();
        }
        Fandc => {
            println!("TODO: FandC");
        }
    }
}