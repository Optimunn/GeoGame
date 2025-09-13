#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod os {
    pub const CONFIG_DIR: &str = ".config/GeoGame";
    pub const CONFIG_FILE: &str = "config.json";
#[cfg(debug_assertions)]
    pub const LOAD_DATA: &str = "data/";
#[cfg(not(debug_assertions))]
    pub const LOAD_DATA: &str = "../Resources/Data/";
#[cfg(debug_assertions)]
    pub const LOAD_IMAGE: &str = "assets/flags/4x3/";
#[cfg(not(debug_assertions))]
    pub const LOAD_IMAGE: &str = "../Resources/Assets/flags/";
#[cfg(debug_assertions)]
    pub const LOAD_ICON: &str = "assets/icons/earth.svg";
#[cfg(not(debug_assertions))]
    pub const LOAD_ICON: &str = "../icons/earth.svg";
}

#[cfg(target_os = "windows")]
pub mod os {
    pub const CONFIG_DIR: &str = "AppData\\Local\\GeoGame";
    pub const CONFIG_FILE: &str = "config.json";
#[cfg(debug_assertions)]
    pub const LOAD_DATA: &str = "data\\";
#[cfg(not(debug_assertions))]
    pub const LOAD_DATA: &str = "Data\\";
#[cfg(debug_assertions)]
    pub const LOAD_IMAGE: &str = "assets\\flags\\4x3\\";
#[cfg(not(debug_assertions))]
    pub const LOAD_IMAGE: &str = "Assets\\flags\\";
#[cfg(debug_assertions)]
    pub const LOAD_ICON: &str = "assets\\icons\\earth.svg";
#[cfg(not(debug_assertions))]
    pub const LOAD_ICON: &str = "..\\icons\\earth.svg";
}

pub mod language {
    pub const EN: &str = "country_en.json";
    pub const RU: &str = "country_ru.json";
}

pub mod url {
    pub const GITHUB: &str = "https://github.com/Optimunn/GeoGame";
    pub const RUST: &str = "https://www.rust-lang.org";
    pub const SLINT: &str = "https://slint.dev";
}

pub mod pallet {
    use slint::Color;

#[allow(dead_code)]
    pub const RED: Color = Color::from_argb_u8(255, 255, 0, 0);
#[allow(dead_code)]
    pub const GREEN: Color = Color::from_argb_u8(255, 0, 255, 0);
#[allow(dead_code)]
    pub const GRAY: Color = Color::from_argb_u8(255, 128, 128, 128);
#[allow(dead_code)]
    pub const FREEDOM: Color = Color::from_argb_encoded(0xffe786d2);
#[allow(dead_code)]
    pub const LAVENDER: Color = Color::from_argb_encoded(0xffab86e7);
#[allow(dead_code)]
    pub const BLUE_SKY: Color = Color::from_argb_encoded(0xff5f79ef);
#[allow(dead_code)]
    pub const MANDARIN: Color = Color::from_argb_encoded(0xffefbd5f);
#[allow(dead_code)]
    pub const RIPE_LIME: Color = Color::from_argb_encoded(0xff45f931);
}

pub mod ui {
#![allow(dead_code)]
//in game timer
    pub const TIME_OUT: i32 = 5;
//links
    pub const LINK_GITHUB: i32 = 1;
    pub const LINK_RUST: i32 = 2;
    pub const LINK_SLINT: i32 = 3;
//game mode
    pub const PLAY_10: i32 = 0;
    pub const PLAY_25: i32 = 1;
    pub const PLAY_HARD: i32 = 2;
//button color
    pub const C_GRAY: i32 = 0;
    pub const C_FREEDOM: i32 = 1;
    pub const C_LAVENDER: i32 = 2;
    pub const C_BLUE_SKY: i32 = 3;
    pub const C_MANDARIN: i32 = 4;
    pub const C_RIPE_LIME: i32 = 5;
//language
    pub const I_EN: i32 = 0;
    pub const I_RU: i32 = 1;

    pub mod scene {
    #![allow(dead_code)]
        pub const GAME_WINDOW: i32 = 0;
        pub const WELCOME_WINDOW: i32 = 1;
        pub const PRE_PLAY_WINDOW: i32 = 2;
        pub const SETTINGS_WINDOW: i32 = 3;
        pub const ABOUT_WINDOW: i32 = 4;
        pub const END_GAME_WINDOW: i32 = 5;
    }
}

#[macro_export]
macro_rules! drop_rc {
    ($model:expr) => {
        ModelRc::from(Rc::new(VecModel::from($model)))
    };
}

#[macro_export]
macro_rules! drop_cell {
    ($model:expr) => {
        Rc::new(Cell::new($model))
    };
}

#[macro_export]
macro_rules! block_checkbox {
    ($checkbox:expr, $len:expr) => {{
        let slice = &$checkbox[..$len];
        let count = slice.iter().filter(|&&x| x).count();
        count <= 1
    }};
}