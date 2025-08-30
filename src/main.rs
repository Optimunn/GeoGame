use slint::{Image, Model, ModelRc, ToSharedString, VecModel};
use std::{cell::Cell, fs, rc::Rc};

use process::{Input, GameLogic, Country};
use uipart::{COLOR_RED, COLOR_GREEN, COLOR_GRAY};

mod process;
mod uipart;
mod loadconfig;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();

    let loaded_config = loadconfig::read_input_config("save/config.json").unwrap();
    let serialized_countries = Input::read_from_file("data/country.json").unwrap();
    

    let continent = GameLogic::create_continents_list(&loaded_config).unwrap();
    let filtered_cont = Input::filter_by_continents(serialized_countries, &continent);
    
    let mut rand_thread = GameLogic::start_rand_to_image();
    let random_number: Rc<Cell<usize>> = GameLogic::get_rand_to_image_cell(&mut rand_thread);
    let random_number_clone = random_number.clone();

    let board_model = update_country(&main_window, &filtered_cont, random_number.get());
    main_window.set_button_data(board_model);


    let _ = main_window.on_button_clicked({
        let main_window_handle = main_window.as_weak();

        move |index| { 
            let main_window = main_window_handle.unwrap();
            let mut model: Vec<ButtonData> = main_window.get_button_data().iter().collect();
            
            let _random_number = random_number_clone.get();

            match index as usize == _random_number {
                true => {
                    model[_random_number].color = COLOR_GREEN;
                }
                false => {
                    model[index as usize].color = COLOR_RED;
                    model[_random_number].color = COLOR_GREEN;
                }
            }

            let board_model = ModelRc::from(Rc::new(VecModel::from(model)));
            main_window.set_button_data(board_model);
        }
    });

    let _ = main_window.on_checkbox_continent_clicked({
        let main_window_handle = main_window.as_weak();

        move || {
            let main_window = main_window_handle.unwrap();
            let mut checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();
        }
    });

    let _ = main_window.on_update_window({
        let main_window_handle = main_window.as_weak();

        move || {
            let main_window = main_window_handle.unwrap();
            
            random_number.set(GameLogic::get_rand_to_image(&mut rand_thread));

            let _random_number = random_number.get();
            let board_model = update_country(&main_window, &filtered_cont, _random_number);
            main_window.set_button_data(board_model);
        }
    });
    main_window.run()
}

fn update_country(main_window: &MainWindow, countries: &[Country], _random_number: usize) -> ModelRc<ButtonData> {
    let mut model: Vec<ButtonData> = main_window.get_button_data().iter().collect();

    let out4 = Input::get_random_countries(countries, 4);
    let patch = GameLogic::get_image_path(&out4[_random_number].flag_4x3);

    let image_data = fs::read(patch).unwrap();
    match Image::load_from_svg_data(&image_data) {
        Ok(image) => main_window.set_loaded_image(image),
        Err(e) => println!("ELI: {}", e),
    }

    for i in 0..4 {
        model[i].color = COLOR_GRAY;
        model[i].text = out4[i].name.to_shared_string();
    }
    ModelRc::from(Rc::new(VecModel::from(model)))
}
