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
    pub fn default() -> InputConfig {
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
    use std::{fs, path::PathBuf};
    use crate::configure::InputConfig;
    #[cfg(not(debug_assertions))]
    use std::path::Path;
    #[cfg(not(debug_assertions))]
    use crate::consts::*;

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
    pub fn load_file_ways() -> (PathBuf, PathBuf, PathBuf) {
        let exe_path: PathBuf = std::env::current_exe().unwrap();
        let exe_dir: &Path = exe_path.parent().unwrap();
        let config_path_string: PathBuf = exe_dir.join(LOAD_CONFIG);
        let data_path_string: PathBuf = exe_dir.join(LOAD_DATA);
        let image_path_string: PathBuf = exe_dir.join(LOAD_IMAGE);
        return (config_path_string, data_path_string, image_path_string);
    }

}

pub mod set {
    use slint::{PhysicalPosition,
        PhysicalSize, ModelRc, VecModel, Color};
    use std::rc::Rc;
    use crate::slint_generatedMainWindow::MainWindow;
    use crate::process::gamelogic;
    use crate::{block_checkbox, drop_rc};

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
        let checkbox_blocked: bool = block_checkbox!(cont, 6);
        if checkbox_blocked { window.set_checkbox_continent_blocked(checkbox_blocked) }
    }
#[inline(always)]
    pub fn checkbox_mode_blocked(window: &MainWindow, mode: &Vec<bool>) {
        let mode_block: bool = block_checkbox!(&mode, 3);
        if mode_block { window.set_checkbox_mode_blocked(mode_block) }
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
}

pub mod get {
    use slint::{Model, Image};
    use crate::slint_generatedMainWindow::MainWindow;
    use crate::process::gamelogic;

#[inline(always)]
    pub fn settings_button_color(window: &MainWindow) -> String {
        let index: i32 = window.get_selected_button_color_index();
        gamelogic::ret_button_color_string(index)
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