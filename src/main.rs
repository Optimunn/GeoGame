use slint::{SharedString, ToSharedString, Weak};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::path::PathBuf;
use std::thread;
use std::cell::Cell;
use std::rc::Rc;

use process::gamelogic;
use consts::*;
use consts::ui::scene;
use configure::configurationsettings as ConfSet;
use configure::{set, get};
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
                        threadfn::load_data_from_thread(
                            &filtered_cont,
                            &mode,
                            &input,
                            &tx_data, #[cfg(not(debug_assertions))] &image_path_string
                        );
                    }
                }
            }
        }
    });
    //? <- Thread

    //*  Randomize countries
    let random_number: Rc<Cell<usize>> = drop_cell!(gamelogic::get_rand_universal(ui::ANSWER_NUM));
    let max_question_number: Rc<Cell<i32>> = drop_cell!(ui::RESET);
    let question_number: Rc<Cell<i32>> = drop_cell!(ui::RESET);

    let mode_selected: Vec<GameMode> = gamelogic::create_mode_list(&loaded_config.mode);
    let _ = Some(tx_cmd.send(ThreadIn {
        mode: Some(mode_selected),
        action: Action::Update,
        checkbox: Some(loaded_config.continents.clone()),
        random: None
    }));

    set::settings_language(&main_window, &loaded_config.language);
    set::settings_button_color(&main_window, &loaded_config.color);
    set::checkbox_continent_blocked(&main_window, &loaded_config.continents);
    set::checkbox_continent_checked(&main_window, loaded_config.continents.clone());
    set::checkbox_mode_blocked(&main_window, &loaded_config.mode);
    set::checkbox_mode_checked(&main_window, loaded_config.mode.clone());

    //* When click on run button
    let _ = main_window.on_run_game_process({
        let tx_cmd_clone: Sender<ThreadIn> = tx_cmd.clone();
        let random_number_clone: Rc<Cell<usize>> = random_number.clone();
        let max_question_number_clone: Rc<Cell<i32>> = max_question_number.clone();
        let question_number_clone: Rc<Cell<i32>> = question_number.clone();

        move |index: i32| {
            random_number_clone.set(gamelogic::get_rand_universal(ui::ANSWER_NUM));
            question_number_clone.set(ui::RESET);

            let number: i32 = match index {
                ui::PLAY_10 => ui::PLAY_10_CNT,
                ui::PLAY_25 => ui::PLAY_25_CNT,
                ui::PLAY_HARD => ui::PLAY_HARD_CNT,
                _ => 0,
            };

            max_question_number_clone.set(number);

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

            match index {
                ui::TIME_OUT => {
                    model.selected = "Time out!".to_shared_string();
                    model.answer = input_names[random_number_get].clone();
                },
                _ => {
                    if index as usize == random_number_get { model.color = pallet::GREEN; }
                    model.selected = input_names[index as usize].clone();
                    model.answer = input_names[random_number_get].clone();
                }
            }
            main_window.set_answer_data(model);
            random_number.set(gamelogic::get_rand_universal(ui::ANSWER_NUM));

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

            if let Ok(input) = rx_data.recv() {
                let q_num: i32 = question_number.get();
                let m_q_num: i32 = max_question_number.get();
                if m_q_num < q_num {
                    let game: EndGame  = EndGame::my_default();
                    set::game_timer_stop(&main_window);
                    set::scene(&main_window, scene::END_GAME_WINDOW);
                    set::end_game_events(&main_window, game);
                    return;
                }
                use GameMode::*;
                match input.mode {
                    Flags => {
                        set::game_window_with_image(&main_window, &input.data.img, input.names);
                    }
                    Capitals => {
                        set::game_window_with_text(&main_window, &input.data.name, input.names);
                    }
                    Fandc => {
                        main_window.set_img_or_text(false);
                        println!("FandC");
                    }
                }
                let question: SharedString = format!("{}/{}", q_num, m_q_num).to_shared_string();
                main_window.set_question_number(question);
                set::game_timer_run(&main_window);
                question_number.set(q_num + 1);
                main_window.set_info_about_country(input.data.to_info());
            }
        }
    });

    //* When click on info button in "About" window
    let _ = main_window.on_open_url_info({
        move |index: i32| {
            match index {
                ui::LINK_GITHUB => open::that(url::GITHUB).unwrap(),
                ui::LINK_RUST => open::that(url::RUST).unwrap(),
                ui::LINK_SLINT => open::that(url::SLINT).unwrap(),
                _ => (),
            }
        }
    });

    //* Select button color
    let _ = main_window.on_selected_button_color({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move |index: i32| {
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