use slint::{Image, Model, ModelRc, PhysicalSize, PhysicalPosition,
    SharedString, ToSharedString, VecModel, Weak};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::cell::Cell;
#[cfg(debug_assertions)]
use std::path::PathBuf;
use std::rc::Rc;

use process::GameLogic;
use consts::*;
use configure::ConfigurationSettings as ConfSet;
use configure::set;
use configure::{InputConfig, Country, Continent};
use threadfn::{ThreadIn, ThreadData, GameMode, Action};

mod process;
mod consts;
mod configure;
mod threadfn;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    //* Drop app window
    let main_window: MainWindow = MainWindow::new().unwrap();
#[cfg(not(debug_assertions))]
    let (config_path_string, data_path_string,  image_path_string) = ConfSet::load_file_ways();

    //*  Load app data
    let serialized_countries: Vec<Country> = match ConfSet::read_from_file(
        #[cfg(debug_assertions)] drop_buf!(LOAD_DATA),
        #[cfg(not(debug_assertions))] &data_path_string)
    {
        Ok(config) => config,
        Err(_) => panic!("Failed to load app data"),
    };

    //*  Load app configuration data
    let mut loaded_config: InputConfig = match ConfSet::read_from_file(
        #[cfg(debug_assertions)] drop_buf!(LOAD_CONFIG),
        #[cfg(not(debug_assertions))] &config_path_string)
    {
        Ok(config) => config,
        Err(_) => InputConfig::default(),
    };

    main_window.window().set_size(set::screen_size(loaded_config.size));
    main_window.window().set_position(set::screen_position(loaded_config.position));
//todo -> load image
    let image_data: Vec<u8> = match std::fs::read("assets/icons/earth.svg") {
        Ok(data) => data,
        Err(_) => panic!("Failed to load image")
    };
    main_window.set_image_welcome(to_img(&image_data));
//todo <-
    let (tx_cmd, rx_cmd): (Sender<ThreadIn>, Receiver<ThreadIn>) = channel();
    let (tx_data, rx_data): (Sender<ThreadData>, Receiver<ThreadData>) = channel();

    //*  Drop thread to filter countries
    //? -> Thread
    thread::spawn({
        let mut filtered_cont: Vec<Country> = Vec::new();
        let mut mode: Vec<GameMode> = Vec::new();

        move || {
            while let Ok(input) = rx_cmd.recv() {
                use Action::*;
                match input.action {
                    Update => {
                        let continent: Vec<Continent> = GameLogic::create_continents_list(&input.checkbox.unwrap());
                        filtered_cont = GameLogic::filter_by_continents(&serialized_countries, &continent);
                        mode = input.mode.unwrap();
                    }
                    Load => {
                        threadfn::load_data_from_thread(&filtered_cont, &mode, &input, &tx_data, #[cfg(not(debug_assertions))] &image_path_string);
                    }
                }
            }
        }
    });
    //? <- Thread

    //*  Randomize countries
    let random_number: Rc<Cell<usize>> = drop_cell!(GameLogic::get_rand_universal(4));

    let mode_selected: Vec<GameMode> = GameLogic::create_mode_list(&loaded_config.mode);
    let _ = Some(tx_cmd.send(ThreadIn {
        mode: Some(mode_selected),
        action: Action::Update,
        checkbox: Some(loaded_config.continents.clone()),
        random: None
    }));

    set::settings_set_button_color(&main_window, &loaded_config.color);
    //* Blocking last checkbox
    set::checkbox_continent_blocked(&main_window, &loaded_config.continents);
    set::checkbox_mode_blocked(&main_window, &loaded_config.mode);
    set::checkbox_continent_checked(&main_window, loaded_config.continents.clone());
    set::checkbox_mode_checked(&main_window, loaded_config.mode.clone());

    //* When click on run button
    let _ = main_window.on_run_game_process({
        let tx_cmd_clone: Sender<ThreadIn> = tx_cmd.clone();
        let random_number_clone: Rc<Cell<usize>> = random_number.clone();

        move |index: i32| {
            random_number_clone.set(GameLogic::get_rand_universal(4));

            let _ = Some(tx_cmd_clone.send(ThreadIn {
                mode: None,
                action: Action::Load,
                checkbox: None,
                random: Some(random_number_clone.get())
            }));
        }
    });

    //* When click on country button
    let _ = main_window.on_button_clicked({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();
        let tx_cmd_clone: Sender<ThreadIn> = tx_cmd.clone();

        move |index: i32| {
            let main_window: MainWindow = main_window_handle.unwrap();
            let input_names: Vec<SharedString> = main_window.get_button_data().iter().collect();
            let mut model: AnswerData = AnswerData {
                answer: "null".to_shared_string(),
                color: pallet::COLOR_RED,
                selected: "null".to_shared_string(),
                visible: true
            };

            let random_number_get: usize = random_number.get();
            if index as usize == random_number_get { model.color = pallet::COLOR_GREEN; }
            model.selected = input_names[index as usize].clone();
            model.answer = input_names[random_number_get].clone();
            main_window.set_answer_data(model);

            random_number.set(GameLogic::get_rand_universal(4));

            let _ = Some(tx_cmd_clone.send(ThreadIn {
                mode: None,
                action: Action::Load,
                checkbox: None,
                random: Some(random_number.get())
            }));
        }
    });

    //* When click on continent checkbox
    let _ = main_window.on_checkbox_clicked({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();
            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();
            let mode: Vec<bool> = main_window.get_checkbox_mode_checked().iter().collect();
            set::checkbox_continent_blocked(&main_window, &checkbox);
            set::checkbox_mode_blocked(&main_window, &mode);

            let mode_selected = GameLogic::create_mode_list(&mode);
            let _ = Some(tx_cmd.send(ThreadIn {
                mode: Some(mode_selected),
                action: Action::Update,
                checkbox: Some(checkbox),
                random: None
            }));
        }
    });

    //* When update window after selected country
    let _ = main_window.on_update_window({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            if let Ok(data) = rx_data.recv() {
                use GameMode::*;
                match data.mode {
                    Flags => {
                        main_window.set_img_or_text(true);
                        main_window.set_loaded_image(to_img(&data.img.unwrap()));
                        main_window.set_button_data(drop_rc!(data.names));
                    }
                    Capitals => {
                        main_window.set_img_or_text(false);
                        main_window.set_loaded_text(data.text.unwrap());
                        main_window.set_button_data(drop_rc!(data.names));
                    }
                    Fandc => {
                        main_window.set_img_or_text(false);
                        println!("FandC");
                    }
                }
            }
        }
    });

    //* When click on info button in "About" window
    let _ = main_window.on_open_url_info({
        move |index: i32| {
            match index {
                1 => open::that(LINK_TO_GITHUB).unwrap(),
                2 => open::that(LINK_TO_RUST).unwrap(),
                3 => open::that(LINK_TO_SLINT).unwrap(),
                _ => (),
            }
        }
    });

    //* Select button color
    let _ = main_window.on_selected_button_color({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move |index| {
            let main_window: MainWindow = main_window_handle.unwrap();
            main_window.set_uniq_button_color(GameLogic::ret_button_color(index));
        }
    });

    //* When close window
    let _ = main_window.window().on_close_requested({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();
            loaded_config.continents = checkbox;
            let mode: Vec<bool> = main_window.get_checkbox_mode_checked().iter().collect();
            loaded_config.mode = mode;
            let window_get_size: PhysicalSize = main_window.window().size();
            loaded_config.size = (window_get_size.width / 2, window_get_size.height / 2);
            let window_get_pos: PhysicalPosition = main_window.window().position();
            loaded_config.position = (window_get_pos.x, position_bug!(window_get_pos.y));
            loaded_config.language = "en".to_string();
            loaded_config.color = set::settings_get_button_color(&main_window);

            ConfSet::write_input_config(#[cfg(debug_assertions)] drop_buf!(LOAD_CONFIG),
                #[cfg(not(debug_assertions))] &config_path_string, &loaded_config).unwrap();
            slint::CloseRequestResponse::HideWindow
        }
    });

    main_window.run()
}

fn to_img(image_data: &[u8]) -> Image {
    match Image::load_from_svg_data(&image_data) {
        Ok(image) => image,
        Err(_) => Image::default(),
    }
}

#[macro_export]
macro_rules! position_bug { // !!! WTF
    ($len:expr) => {
        $len + 56
    };
}