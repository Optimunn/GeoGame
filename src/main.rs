use slint::{Image, Model, ModelRc, SharedString, ToSharedString, VecModel, Weak};
use std::cell::{Cell, RefCell};
#[cfg(not(debug_assertions))]
use std::path::Path;
use std::path::PathBuf;
use std::{fs, rc::Rc};
use rand::rngs::ThreadRng;

use process::GameLogic;
use consts::*;
use configure::ConfigurationSettings as ConfSet;
use configure::{InputConfig, Country, Continent};

mod process;
mod consts;
mod configure;

slint::include_modules!();

struct ThreadData {
    img: Image,
    country: ModelRc<SharedString>
}

fn main() -> Result<(), slint::PlatformError> {
    //* Drop app window
    let main_window: MainWindow = MainWindow::new().unwrap();

#[cfg(not(debug_assertions))]
    let exe_path: PathBuf = std::env::current_exe().unwrap();
#[cfg(not(debug_assertions))]
    let exe_dir: &Path = exe_path.parent().unwrap();
#[cfg(not(debug_assertions))]
    let config_path_string: PathBuf = exe_dir.join(LOAD_CONFIG);
#[cfg(not(debug_assertions))]
    let data_path_string: PathBuf = exe_dir.join(LOAD_DATA);
#[cfg(not(debug_assertions))]
    let image_path_string: PathBuf = exe_dir.join(LOAD_IMAGE);

    //*  Load app configuration data
    let mut loaded_config: InputConfig = match ConfSet::read_from_file(
            #[cfg(debug_assertions)] path_buf!(LOAD_CONFIG), #[cfg(not(debug_assertions))] &config_path_string) {
        Ok(config) => config,
        Err(_) => InputConfig { continents: vec![true; 6] },
    };
    //*  Load app data
    let serialized_countries: Vec<Country> = match ConfSet::read_from_file(
            #[cfg(debug_assertions)] path_buf!(LOAD_DATA), #[cfg(not(debug_assertions))] &data_path_string) {
        Ok(config) => config,
        Err(_) => panic!("Failed to load app data"),
    };
    
    //* Get available continents list
    let continent: Vec<Continent> = GameLogic::create_continents_list(&loaded_config.continents).unwrap();
    let filtered_cont: Vec<Country> = GameLogic::filter_by_continents(&serialized_countries, &continent);
    let filtered_cont: Rc<RefCell<Vec<Country>>> = Rc::new(RefCell::new(filtered_cont));
    
    //*  Randomize countries
    let mut rand_thread: ThreadRng = GameLogic::start_rand_to_image();
    let random_number: Rc<Cell<usize>> = GameLogic::get_rand_to_image_cell(&mut rand_thread);

    //* Blocking last checkbox
    let checkbox_blocked: bool = block_checkbox!(&loaded_config.continents, 6);
    if checkbox_blocked { main_window.set_checkbox_continent_blocked(checkbox_blocked) }

    let checkbox_model: ModelRc<bool> = simplified_rc!(loaded_config.continents.clone());
    main_window.set_checkbox_continent_checked(checkbox_model);

    //* Update flags in application
    let data: ThreadData = update_country(&filtered_cont.borrow(), random_number.get(), #[cfg(not(debug_assertions))] &image_path_string);
    main_window.set_loaded_image(data.img);
    main_window.set_button_data(data.country);

    let _ = main_window.on_button_clicked({
        //* When click on country button
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();
        let random_number_clone: Rc<Cell<usize>> = random_number.clone();

        move |index| { 
            let main_window: MainWindow = main_window_handle.unwrap();
            let input_names: Vec<SharedString> = main_window.get_button_data().iter().collect();
            let mut model: AnswerData = AnswerData {
                answer: "null".to_shared_string(),
                color: COLOR_RED,
                selected: "null".to_shared_string(),
                visible: true
            };
            
            let _random_number: usize = random_number_clone.get();
            if index as usize == _random_number { model.color = COLOR_GREEN; }
            model.selected = input_names[index as usize].clone();
            model.answer = input_names[_random_number].clone();
            main_window.set_answer_data(model);
        }
    });

    let _ = main_window.on_checkbox_continent_clicked({
        //* When click on continent checkbox
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();
        let filtered_cont: Rc<RefCell<Vec<Country>>> = Rc::clone(&filtered_cont);

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();
            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();
#[cfg(debug_assertions)]
            println!("{:?}", checkbox);
            let blocked: bool = block_checkbox!(checkbox, 6);
            let continent: Vec<Continent> = GameLogic::create_continents_list(&checkbox).unwrap();
            *filtered_cont.borrow_mut() = GameLogic::filter_by_continents(&serialized_countries, &continent);
            main_window.set_checkbox_continent_blocked(blocked);
        }
    });

    let _ = main_window.on_update_window({
        //* When update window after selected country
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            random_number.set(GameLogic::get_rand_to_image(&mut rand_thread));
            let _random_number: usize = random_number.get();
            let data: ThreadData = update_country(&filtered_cont.borrow(), _random_number, #[cfg(not(debug_assertions))] &image_path_string);
            main_window.set_loaded_image(data.img);
            main_window.set_button_data(data.country);
        }
    });

    let _ = main_window.on_open_url_info({
        //* When click on info button in "About" window
        move |item| {
            match item {
                1 => open::that(LINK_TO_GITHUB).unwrap(),
                2 => open::that(LINK_TO_RUST).unwrap(),
                3 => open::that(LINK_TO_SLINT).unwrap(),
                _ => (),
            }
        }
    });

    let _ = main_window.window().on_close_requested({
        //* When close window
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();
            loaded_config.continents = checkbox;

            ConfSet::write_input_config(#[cfg(debug_assertions)] path_buf!(LOAD_CONFIG),
                #[cfg(not(debug_assertions))] &config_path_string, &loaded_config).unwrap();
            slint::CloseRequestResponse::HideWindow
        }
    });

    main_window.run()
}

fn update_country(countries: &[Country], _random_number: usize,
        #[cfg(not(debug_assertions))] _image_patch: &PathBuf) -> ThreadData {
    let mut model: Vec<SharedString> = vec![SharedString::new(); 4];

    let out4: Vec<Country> = GameLogic::get_random_countries(countries, 4);
#[cfg(debug_assertions)]
    let patch: String = out4[_random_number].flag_4x3.to_string();
#[cfg(debug_assertions)]
    let patch: String = format!("{LOAD_IMAGE}{}", patch);
#[cfg(not(debug_assertions))]
    let patch: PathBuf = _image_patch.join(out4[_random_number].flag_4x3.as_str());

    let image_data: Vec<u8> = fs::read(patch).unwrap();
    let out_image: Image = match Image::load_from_svg_data(&image_data) {
        Ok(image) => image,
        Err(_) => Image::default(),
    };
    
    for i in 0..4 { model[i] = out4[i].name.to_shared_string(); }
    let board_model: ModelRc<SharedString> = simplified_rc!(model);

    ThreadData {
        img: out_image,
        country: board_model
    }
}