use serde::{Deserialize, Serialize};

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
    pub flag_4x3: String,
#[allow(dead_code)]
    iso: bool,
#[allow(dead_code)]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputConfig {
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub continents: Vec<bool>,
    pub mode: Vec<bool>,
    pub language: String,
    pub color: String,
}

impl InputConfig {
    pub fn default() -> Self {
        InputConfig {
            size: (500, 500),
            position: (0, 0),
            continents: vec![true; 6],
            mode: vec![true, false, false],
            language: "en".to_string(),
            color: "gray".to_string(),
        }
    }
}

pub mod configurationsettings {
    use serde::de::DeserializeOwned;
    use serde_json::Result;
    use std::path::PathBuf;
    use std::fs;
    use crate::configure::InputConfig;
    use crate::consts::os::*;

    pub fn input_config_path() -> PathBuf {
        let home_dir: PathBuf = match std::env::home_dir(){
            Some(patch) => patch,
            None => panic!("Failed to get home directory!"),
        };
        let config_dir: PathBuf = home_dir.join(CONFIG_DIR);
        if !config_dir.is_dir() {
            fs::create_dir(&config_dir)
                .expect("Failed to create config directory!");
        }
        config_dir.join(CONFIG_FILE)
    }

    pub fn input_data_path(language: &str, #[cfg(not(debug_assertions))]  patch: &PathBuf) -> PathBuf {
    #[cfg(debug_assertions)] {
            let patch_dbg: String = format!("{LOAD_DATA}{}", language);
            PathBuf::from(patch_dbg.to_string())
        }
    #[cfg(not(debug_assertions))]
        patch.join(language)
    }

	pub fn read_from_file<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
        let data: String = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(_) => String::from(""),
        };
        let result = serde_json::from_str(&data)?;
        Ok(result)
    }

    pub fn write_input_config(path: &PathBuf, input: &InputConfig) -> Result<()> {
        let file: fs::File = fs::File::create(path).unwrap();
        let output = serde_json::to_writer_pretty(file, input)?;
        Ok(output)
    }
#[cfg(not(debug_assertions))]
    pub fn load_file_ways() -> (PathBuf, PathBuf) {
        use std::path::Path;
        let exe_path: PathBuf = std::env::current_exe().unwrap();
        let exe_dir: &Path = exe_path.parent().unwrap();
        let data_path_string: PathBuf = exe_dir.join(LOAD_DATA);
        let image_path_string: PathBuf = exe_dir.join(LOAD_IMAGE);
        return (data_path_string, image_path_string);
    }

}

pub mod set {
    use slint::{PhysicalPosition, SharedString,
        PhysicalSize, ModelRc, VecModel, Color};
#[cfg(not(debug_assertions))]
    use std::path::PathBuf;
    use std::fs;
    use std::rc::Rc;
    use crate::slint_generatedMainWindow::MainWindow;
    use crate::process::gamelogic;
    use crate::{block_checkbox, drop_rc};

#[inline(always)]
    pub fn scene(window: &MainWindow, scene: i32) {
        window.set_scene_visible(scene);
    }
#[inline(always)]
    pub fn screen_size(size: (u32, u32)) -> PhysicalSize {
        PhysicalSize::new(size.0, size.1)
    }
#[inline(always)]
    pub fn screen_position(position: (i32, i32)) -> PhysicalPosition {
        PhysicalPosition::new(position.0, position.1)
    }
#[inline(always)]
    pub fn checkbox_continent_blocked(window: &MainWindow, cont: &Vec<bool>) {
        let checkbox_blocked: bool = block_checkbox!(&cont, 6);
        window.set_checkbox_continent_blocked(checkbox_blocked)
    }
#[inline(always)]
    pub fn game_timer_stop(window: &MainWindow) {
        window.set_run_game_timer(false);
    }
#[inline(always)]
    pub fn game_timer_run(window: &MainWindow) {
        window.set_run_game_timer(true);
    }
#[inline(always)]
    pub fn checkbox_mode_blocked(window: &MainWindow, mode: &Vec<bool>) {
        let mode_block: bool = block_checkbox!(&mode, 3);
        window.set_checkbox_mode_blocked(mode_block)
    }
#[inline(always)]
    pub fn checkbox_continent_checked(window: &MainWindow, cont: Vec<bool>) {
        let checkbox_model: ModelRc<bool> = drop_rc!(cont);
        window.set_checkbox_continent_checked(checkbox_model);
    }
#[inline(always)]
    pub fn checkbox_mode_checked(window: &MainWindow, mode: Vec<bool>) {
        let mode_model: ModelRc<bool> = drop_rc!(mode);
        window.set_checkbox_mode_checked(mode_model);
    }
#[inline(always)]
    pub fn settings_button_color(window: &MainWindow, color: &String) {
        let index: i32 = gamelogic::ret_button_color_index(color);
        let color: Color = gamelogic::ret_button_color(index);
        window.set_selected_button_color_index(index);
        window.set_uniq_button_color(color);
    }
#[inline(always)]
    pub fn settings_language(window: &MainWindow, lang: &String) {
        let index: i32 = gamelogic::ret_language_index(lang);
        window.set_selected_language_index(index);
    }
#[inline(always)]
    pub fn game_window_with_image(window: &MainWindow, data: &[u8], model: Vec<SharedString>) {
        use crate::configure::get::img;
        window.set_img_or_text(true);
        window.set_loaded_image(img(data));
        window.set_button_data(drop_rc!(model));
    }
#[inline(always)]
    pub fn game_window_no_image(window: &MainWindow, text: SharedString, model: Vec<SharedString>) {
        window.set_img_or_text(false);
        window.set_loaded_text(text);
        window.set_button_data(drop_rc!(model));
    }

    pub fn image_welcome(window: &MainWindow, #[cfg(not(debug_assertions))] patch: &PathBuf) {
        use crate::consts::os::LOAD_ICON;
        use crate::configure::get::img;
    #[cfg(debug_assertions)]
        let welcome_patch: String = LOAD_ICON.to_string();
    #[cfg(not(debug_assertions))]
        let welcome_patch: PathBuf = patch.join(LOAD_ICON);
        let image_data: Vec<u8> = match fs::read(welcome_patch) {
            Ok(data) => data,
            Err(_) => panic!("Failed to load image")
        };
        window.set_image_welcome(img(&image_data));
    }
}

macro_rules! position_bug { // !!! WTF
    ($len:expr) => {
        $len + 56
    };
}

pub mod get {
    use slint::{Model, Image, PhysicalPosition, PhysicalSize, SharedString};
    use crate::slint_generatedMainWindow::MainWindow;
    use crate::process::gamelogic;

#[inline(always)]
    pub fn window_size(size: PhysicalSize) -> (u32, u32) {
        (size.width / 2, size.height / 2)
    }
#[inline(always)]
    pub fn window_position(position: PhysicalPosition) -> (i32, i32) {
        (position.x, position_bug!(position.y))
    }
#[inline(always)]
    pub fn button_data(window: &MainWindow) -> Vec<SharedString> {
        window.get_button_data().iter().collect()
    }
#[inline(always)]
    pub fn settings_button_color(window: &MainWindow) -> String {
        let index: i32 = window.get_selected_button_color_index();
        gamelogic::ret_button_color_string(index)
    }
#[inline(always)]
    pub fn settings_language(window: &MainWindow) -> String {
        let index: i32 = window.get_selected_language_index();
        gamelogic::ret_language_string(index)
    }
#[inline(always)]
    pub fn settings_language_patch(lang: &String) -> &'static str {
        let index: i32 = gamelogic::ret_language_index(lang);
        gamelogic::ret_language(index)
    }
#[inline(always)]
    pub fn checkbox_continent_checked(window: &MainWindow) -> Vec<bool> {
        window.get_checkbox_continent_checked().iter().collect()
    }
#[inline(always)]
    pub fn checkbox_mode_checked(window: &MainWindow) -> Vec<bool> {
        window.get_checkbox_mode_checked().iter().collect()
    }

    pub fn img(image_data: &[u8]) -> Image {
        match Image::load_from_svg_data(&image_data) {
            Ok(image) => image,
            Err(_) => panic!("Failed to load image"),
        }
    }
}