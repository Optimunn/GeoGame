use slint::{Image, Model, ModelRc, SharedString, ToSharedString, VecModel, Weak};
use rand::rngs::ThreadRng;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::cell::Cell;
#[cfg(not(debug_assertions))]
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::rc::Rc;

use process::GameLogic;
use consts::*;
use configure::ConfigurationSettings as ConfSet;
use configure::{InputConfig, Country, Continent, GameMode, Action};

mod process;
mod consts;
mod configure;

slint::include_modules!();

struct ThreadData {
    mode: GameMode,
    img: Option<Vec<u8>>,
    text: Option<SharedString>,
    names: Vec<SharedString>
}

struct ThreadIn {
    mode: Option<Vec<GameMode>>,
    action: Action,
    checkbox: Option<Vec<bool>>,
    random: Option<usize>
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
            #[cfg(debug_assertions)] drop_buf!(LOAD_CONFIG), #[cfg(not(debug_assertions))] &config_path_string) {
        Ok(config) => config,
        Err(_) => InputConfig {
            continents: vec![true; 6],
            mode: vec![true; 3]
        },
    };
    //*  Load app data
    let serialized_countries: Vec<Country> = match ConfSet::read_from_file(
            #[cfg(debug_assertions)] drop_buf!(LOAD_DATA), #[cfg(not(debug_assertions))] &data_path_string) {
        Ok(config) => config,
        Err(_) => panic!("Failed to load app data"),
    };

    let (tx_cmd, rx_cmd): (Sender<ThreadIn>, Receiver<ThreadIn>) = channel();
    let (tx_data, rx_data): (Sender<ThreadData>, Receiver<ThreadData>) = channel();

    //*  Drop thread to filter countries
    thread::spawn({
        let mut filtered_cont: Vec<Country> = Vec::new();
        let mut mode: Vec<GameMode> = Vec::new();

        move || {
            while let Ok(input) = rx_cmd.recv() {
                use Action::*;
                if input.action == Update || input.action == Init {
                    let continent: Vec<Continent> = GameLogic::create_continents_list(&input.checkbox.unwrap());
                    filtered_cont = GameLogic::filter_by_continents(&serialized_countries, &continent);
                    mode = input.mode.unwrap();
                }
                if input.action == Load || input.action == Init {
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
                            let image_data: Vec<u8> = fs::read(patch).unwrap(); //todo: File read warning
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
            }
        }
    });

    //*  Randomize countries
    let random_number: Rc<Cell<usize>> = drop_cell!(GameLogic::get_rand_universal(4));

    let mode_selected = GameLogic::create_mode_list(&loaded_config.mode);
    let _ = Some(tx_cmd.send(ThreadIn {
        mode: Some(mode_selected),
        action: Action::Update,
        checkbox: Some(loaded_config.continents.clone()),
        random: None
    }));

    //* Blocking last checkbox
    let checkbox_blocked: bool = block_checkbox!(&loaded_config.continents, 6);
    if checkbox_blocked { main_window.set_checkbox_continent_blocked(checkbox_blocked) }
    let mode_block: bool = block_checkbox!(&loaded_config.mode, 3);
    if mode_block { main_window.set_checkbox_mode_blocked(mode_block) }

    let checkbox_model: ModelRc<bool> = drop_rc!(loaded_config.continents.clone());
    main_window.set_checkbox_continent_checked(checkbox_model);
    let mode_model: ModelRc<bool> = drop_rc!(loaded_config.mode.clone());
    main_window.set_checkbox_mode_checked(mode_model);

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
                color: COLOR_RED,
                selected: "null".to_shared_string(),
                visible: true
            };

            let random_number_get: usize = random_number.get();
            if index as usize == random_number_get { model.color = COLOR_GREEN; }
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
            let blocked: bool = block_checkbox!(checkbox, 6);
            let mode_blocked: bool = block_checkbox!(mode, 3);

			main_window.set_checkbox_continent_blocked(blocked);
            main_window.set_checkbox_mode_blocked(mode_blocked);

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

    //* When close window
    let _ = main_window.window().on_close_requested({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();
            let mode: Vec<bool> = main_window.get_checkbox_mode_checked().iter().collect();
            loaded_config.continents = checkbox;
            loaded_config.mode = mode;

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