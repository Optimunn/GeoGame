use slint::{Image, Model, ModelRc, SharedString, ToSharedString, VecModel, Weak};
use rand::rngs::ThreadRng;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
#[cfg(not(debug_assertions))]
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::rc::Rc;

use process::GameLogic;
use consts::*;
use configure::ConfigurationSettings as ConfSet;
use configure::{InputConfig, Country, Continent};

mod process;
mod consts;
mod configure;

slint::include_modules!();

struct ThreadData {
    img: Vec<u8>,
    country: Vec<SharedString>
}

#[derive(PartialEq)]
enum Action {
    Init,
    Update,
    Load
}

struct ThreadIn {
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

    let (tx_cmd, rx_cmd): (Sender<ThreadIn>, Receiver<ThreadIn>) = channel();
    let (tx_data, rx_data): (Sender<ThreadData>, Receiver<ThreadData>) = channel();
    
    //*  Drop thread to filter countries
    thread::spawn({
        let mut filtered_cont: Vec<Country> = Vec::new();
        
        move || {
            while let Ok(input) = rx_cmd.recv() {
                use Action::*;
                if input.action == Update || input.action == Init {
                    let continent: Vec<Continent> = GameLogic::create_continents_list(&input.checkbox.unwrap()).unwrap();
                    filtered_cont = GameLogic::filter_by_continents(&serialized_countries, &continent);
                }
                if input.action == Load || input.action == Init {
                    let mut model: Vec<SharedString> = vec![SharedString::new(); 4];

                    let out4: Vec<Country> = GameLogic::get_random_countries(&filtered_cont, 4);
                #[cfg(debug_assertions)]
                    let patch: String = out4[input.random.unwrap()].flag_4x3.to_string();
                #[cfg(debug_assertions)]
                    let patch: String = format!("{LOAD_IMAGE}{}", patch);
                #[cfg(not(debug_assertions))]
                    let patch: PathBuf = image_path_string.join(out4[input.random].flag_4x3.as_str());
                    let image_data: Vec<u8> = fs::read(patch).unwrap(); //todo: File read warning
                    
                    for i in 0..4 { model[i] = out4[i].name.to_shared_string(); }

                    let data: ThreadData = ThreadData { img: image_data, country: model };
                    tx_data.send(data).unwrap();
                }
            }
        }
    });
    
    //*  Randomize countries
    let mut rand_thread: ThreadRng = GameLogic::start_rand_to_image();
    let mut random_number: usize = GameLogic::get_rand_universal(&mut rand_thread);

    #[allow(unused_must_use)]
    tx_cmd.send(ThreadIn {
        action: Action::Update,
        checkbox: Some(loaded_config.continents.clone()),
        random: None
    });

    //* Blocking last checkbox
    let checkbox_blocked: bool = block_checkbox!(&loaded_config.continents, 6);
    if checkbox_blocked { main_window.set_checkbox_continent_blocked(checkbox_blocked) }

    let checkbox_model: ModelRc<bool> = simplified_rc!(loaded_config.continents.clone());
    main_window.set_checkbox_continent_checked(checkbox_model);

    let _ = main_window.on_run_game_process({
        let tx_cmd_clone: Sender<ThreadIn> = tx_cmd.clone();
        
        #[allow(unused_variables)]
        move |index| {
            println!("index: {}", index);
            #[allow(unused_must_use)]
            tx_cmd_clone.send(ThreadIn {
                action: Action::Load,
                checkbox: None,
                random: Some(random_number)
            });
        }
    });

    //* When click on country button
    let _ = main_window.on_button_clicked({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();
        let tx_cmd_clone: Sender<ThreadIn> = tx_cmd.clone();

        move |index| { 
            let main_window: MainWindow = main_window_handle.unwrap();
            let input_names: Vec<SharedString> = main_window.get_button_data().iter().collect();
            let mut model: AnswerData = AnswerData {
                answer: "null".to_shared_string(),
                color: COLOR_RED,
                selected: "null".to_shared_string(),
                visible: true
            };
            
            if index as usize == random_number { model.color = COLOR_GREEN; }
            model.selected = input_names[index as usize].clone();
            model.answer = input_names[random_number].clone();
            main_window.set_answer_data(model);

            random_number = GameLogic::get_rand_universal(&mut rand_thread);

            #[allow(unused_must_use)]
            tx_cmd_clone.send(ThreadIn {
                action: Action::Load,
                checkbox: None,
                random: Some(random_number)
            });
        }
    });

    //* When click on continent checkbox
    let _ = main_window.on_checkbox_continent_clicked({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();
            let checkbox: Vec<bool> = main_window.get_checkbox_continent_checked().iter().collect();
            let blocked: bool = block_checkbox!(checkbox, 6);

            main_window.set_checkbox_continent_blocked(blocked);

            #[allow(unused_must_use)]
            tx_cmd.send(ThreadIn {
                action: Action::Update,
                checkbox: Some(checkbox),
                random: None
            });
        }
    });

    //* When update window after selected country
    let _ = main_window.on_update_window({
        let main_window_handle: Weak<MainWindow> = main_window.as_weak();

        move || {
            let main_window: MainWindow = main_window_handle.unwrap();

            if let Ok(data) = rx_data.recv() {
                main_window.set_loaded_image(to_img(&data.img));
                main_window.set_button_data(simplified_rc!(data.country));
            }
        }
    });

    //* When click on info button in "About" window
    let _ = main_window.on_open_url_info({
        move |index| {
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
            loaded_config.continents = checkbox;

            ConfSet::write_input_config(#[cfg(debug_assertions)] path_buf!(LOAD_CONFIG),
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