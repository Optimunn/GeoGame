use slint::{Image, Model, ModelRc, ToSharedString, VecModel, Weak};
use std::cell::{Cell, RefCell};
use std::{fs, rc::Rc};
use rand::rngs::ThreadRng;

use process::GameLogic;
use uipart::{COLOR_RED, COLOR_GREEN, COLOR_GRAY};
use configure::ConfigurationSettings as ConfSet;
use configure::{InputConfig, Country, Continent};

mod process;
mod uipart;
mod configure;

slint::include_modules!();

macro_rules! simplified_rc {
    ($model:expr) => {
        ModelRc::from(Rc::new(VecModel::from($model)))
    };
}

macro_rules! block_checkbox {
    ($checkbox:expr, $len:expr) => {{
        let slice = &$checkbox[..$len];
        let count = slice.iter().filter(|&&x| x).count();
        count <= 1
    }};
}

const LOAD_CONFIG: &str = "save/config.json";
const LOAD_DATA: &str = "data/country.json";

fn main() -> Result<(), slint::PlatformError> {
    let main_window: MainWindow = MainWindow::new().unwrap();

    let loaded_config: InputConfig = match ConfSet::read_from_file(LOAD_CONFIG) {
        Ok(config) => config,
        Err(_) => InputConfig { continents: vec![true; 6] },
    };
    let serialized_countries: Vec<Country> = match ConfSet::read_from_file(LOAD_DATA) {
        Ok(config) => config,
        Err(_) => panic!("Failed to load app data"),
    };
    
    let continent: Vec<Continent> = GameLogic::create_continents_list(&loaded_config.continents).unwrap();
    let filtered_cont: Vec<Country> = GameLogic::filter_by_continents(&serialized_countries, &continent);
    let filtered_cont: Rc<RefCell<Vec<Country>>> = Rc::new(RefCell::new(filtered_cont));
    
    let mut rand_thread: ThreadRng = GameLogic::start_rand_to_image();
    let random_number: Rc<Cell<usize>> = GameLogic::get_rand_to_image_cell(&mut rand_thread);

    let checkbox_blocked: bool = block_checkbox!(&loaded_config.continents, 6);
    if checkbox_blocked { main_window.set_checkbox_continent_blocked(checkbox_blocked) }

    let checkbox_model: ModelRc<bool> = simplified_rc!(loaded_config.continents);
    main_window.set_checkbox_continent_checked(checkbox_model);

    let board_model: ModelRc<ButtonData> = update_country(&main_window, &filtered_cont.borrow(), random_number.get());
    main_window.set_button_data(board_model);

    let _ = main_window.on_button_clicked({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();
        let random_number_clone: Rc<Cell<usize>> = random_number.clone();

        move |index| { 
            let main_window: MainWindow = main_window_handle.unwrap();
            let mut model: Vec<ButtonData> = main_window.get_button_data().iter().collect();
            
            let _random_number: usize = random_number_clone.get();

            match index as usize == _random_number {
                true => {
                    model[_random_number].color = COLOR_GREEN;
                }
                false => {
                    model[index as usize].color = COLOR_RED;
                    model[_random_number].color = COLOR_GREEN;
                }
            }
            main_window.set_button_data(simplified_rc!(model));
        }
    });

    let _ = main_window.on_checkbox_continent_clicked({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();
        let filtered_cont: Rc<RefCell<Vec<Country>>> = Rc::clone(&filtered_cont);

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();
            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();

            let blocked: bool = block_checkbox!(checkbox, 6);
            let continent: Vec<Continent> = GameLogic::create_continents_list(&checkbox).unwrap();
            *filtered_cont.borrow_mut() = GameLogic::filter_by_continents(&serialized_countries, &continent);
            main_window.set_checkbox_continent_blocked(blocked);
        }
    });

    let _ = main_window.on_update_window({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            random_number.set(GameLogic::get_rand_to_image(&mut rand_thread));
            let _random_number: usize = random_number.get();
            let board_model: ModelRc<ButtonData> = update_country(&main_window, &filtered_cont.borrow(), _random_number);

            main_window.set_button_data(board_model);
        }
    });

    let _ = main_window.window().on_close_requested({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();

            ConfSet::write_input_config(LOAD_CONFIG, &InputConfig { continents: checkbox }).unwrap();
            slint::CloseRequestResponse::HideWindow
        }
    });

    main_window.run()
}

fn update_country(main_window: &MainWindow, countries: &[Country], _random_number: usize) -> ModelRc<ButtonData> {
    let mut model: Vec<ButtonData> = main_window.get_button_data().iter().collect();

    let out4: Vec<Country> = GameLogic::get_random_countries(countries, 4);
    let patch: String = GameLogic::get_image_path(&out4[_random_number].flag_4x3);

    let image_data: Vec<u8> = fs::read(patch).unwrap();
    match Image::load_from_svg_data(&image_data) {
        Ok(image) => main_window.set_loaded_image(image),
        Err(e) => println!("ELI: {}", e),
    }

    for i in 0..4 {
        model[i].color = COLOR_GRAY;
        model[i].text = out4[i].name.to_shared_string();
    }
    simplified_rc!(model)
}