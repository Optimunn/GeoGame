use slint::{ModelRc, SharedString, ToSharedString, VecModel, Weak};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::path::PathBuf;
use std::thread;
use std::cell::Cell;
use std::rc::Rc;

use process::gamelogic;
use consts::*;
use configure::configurationsettings as ConfSet;
use configure::{set, get};
use configure::{InputConfig, Country, Continent};
use threadfn::{ThreadIn, ThreadData, GameMode, Action};

mod process;
mod consts;
mod configure;
mod threadfn;

slint::include_modules!();

impl AnswerData {
    fn my_default() -> Self {
        AnswerData {
            answer: "null".to_shared_string(),
            color: pallet::RED,
            selected: "null".to_shared_string(),
            visible: true
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    //* Drop app window
    let main_window: MainWindow = MainWindow::new().unwrap();

    #[cfg(not(debug_assertions))]
    let (data_path_string,  image_path_string) = ConfSet::load_file_ways();

    //*  Load app configuration data
    let conf_settings = ConfSet::input_config_path();
    let mut loaded_config: InputConfig = match ConfSet::read_from_file(&conf_settings)
    {
        Ok(config) => config,
        Err(_) => InputConfig::default(),
    };

    let config_language: &str = get::settings_language_patch(&loaded_config.language);
    let load_path: PathBuf = ConfSet::input_data_path(config_language, #[cfg(not(debug_assertions))] &data_path_string);

    //*  Load app data
    let serialized_countries: Vec<Country> = match ConfSet::read_from_file(&load_path)
    {
        Ok(config) => config,
        Err(_) => panic!("Failed to load app data"),
    };

    main_window.window().set_size(set::screen_size(loaded_config.size));
    main_window.window().set_position(set::screen_position(loaded_config.position));

    set::image_welcome(&main_window, #[cfg(not(debug_assertions))] &image_path_string);

    let (tx_cmd, rx_cmd): (Sender<ThreadIn>, Receiver<ThreadIn>) = channel();
    let (tx_data, rx_data): (Sender<ThreadData>, Receiver<ThreadData>) = channel();

    //*  Drop thread to filter countries
    //? -> Thread
    let _ = thread::spawn({
        let mut filtered_cont: Vec<Country> = Vec::new();
        let mut mode: Vec<GameMode> = Vec::new();

        move || {
            while let Ok(input) = rx_cmd.recv() {
                use Action::*;
                match input.action {
                    Update => {
                        let continent: Vec<Continent> = gamelogic::create_continents_list(&input.checkbox.unwrap());
                        filtered_cont = gamelogic::filter_by_continents(&serialized_countries, &continent);
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
    let random_number: Rc<Cell<usize>> = drop_cell!(gamelogic::get_rand_universal(4));

    let mode_selected: Vec<GameMode> = gamelogic::create_mode_list(&loaded_config.mode);
    let _ = Some(tx_cmd.send(ThreadIn {
        mode: Some(mode_selected),
        action: Action::Update,
        checkbox: Some(loaded_config.continents.clone()),
        random: None
    }));

    set::settings_language(&main_window, &loaded_config.language);
    set::settings_button_color(&main_window, &loaded_config.color);
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
            random_number_clone.set(gamelogic::get_rand_universal(4));

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
            let input_names: Vec<SharedString> = get::button_data(&main_window);
            let mut model: AnswerData = AnswerData::my_default();

            let random_number_get: usize = random_number.get();
            if index as usize == random_number_get { model.color = pallet::GREEN; }
            model.selected = input_names[index as usize].clone();
            model.answer = input_names[random_number_get].clone();
            main_window.set_answer_data(model);

            random_number.set(gamelogic::get_rand_universal(4));

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
            let checkbox: Vec<bool> = get::checkbox_continent_checked(&main_window);
            let mode: Vec<bool> = get::checkbox_mode_checked(&main_window);
            set::checkbox_continent_blocked(&main_window, &checkbox);
            set::checkbox_mode_blocked(&main_window, &mode);

            let mode_selected: Vec<GameMode> = gamelogic::create_mode_list(&mode);
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
                        main_window.set_loaded_image(get::img(&data.img.unwrap()));
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
                1 => open::that(url::GITHUB).unwrap(),
                2 => open::that(url::RUST).unwrap(),
                3 => open::that(url::SLINT).unwrap(),
                _ => (),
            }
        }
    });

    //* Select button color
    let _ = main_window.on_selected_button_color({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move |index| {
            let main_window: MainWindow = main_window_handle.unwrap();
            main_window.set_uniq_button_color(gamelogic::ret_button_color(index));
        }
    });

    //* When close window
    let _ = main_window.window().on_close_requested({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            loaded_config.size = get::window_size(main_window.window().size());
            loaded_config.position = get::window_position(main_window.window().position());
            loaded_config.continents = get::checkbox_continent_checked(&main_window);
            loaded_config.mode = get::checkbox_mode_checked(&main_window);
            loaded_config.language = get::settings_language(&main_window);
            loaded_config.color = get::settings_button_color(&main_window);

            ConfSet::write_input_config(&conf_settings, &loaded_config).unwrap();
            slint::CloseRequestResponse::HideWindow
        }
    });

    main_window.run()
}