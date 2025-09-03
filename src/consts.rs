#[cfg(debug_assertions)]
pub const LOAD_CONFIG: &str = "save/config.json";
#[cfg(debug_assertions)]
pub const LOAD_DATA: &str = "data/country_en.json";
#[cfg(debug_assertions)]
pub const LOAD_IMAGE: &str = "assets/flags/4x3/";

#[cfg(not(debug_assertions))]
pub const LOAD_CONFIG: &str = "../Resources/Save/config.json";
#[cfg(not(debug_assertions))]
pub const LOAD_DATA: &str = "../Resources/Data/country_en.json";
#[cfg(not(debug_assertions))]
pub const LOAD_IMAGE: &str = "../Assets/flags/";


use slint::{Color};

#[allow(dead_code)]
pub const COLOR_RED: Color = Color::from_argb_u8(255, 255, 0, 0);
#[allow(dead_code)]
pub const COLOR_GREEN: Color = Color::from_argb_u8(255, 0, 255, 0);
#[allow(dead_code)]
pub const COLOR_YELLOW: Color = Color::from_argb_u8(255, 255, 255, 0);
#[allow(dead_code)]
pub const COLOR_BLUE: Color = Color::from_argb_u8(255, 0, 0, 255);
#[allow(dead_code)]
pub const COLOR_GRAY: Color = Color::from_argb_u8(255, 128, 128, 128);


#[macro_export]
macro_rules! simplified_rc {
    ($model:expr) => {
        ModelRc::from(Rc::new(VecModel::from($model)))
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

#[macro_export]
macro_rules! path_buf {
    ($str:expr) => {
        &PathBuf::from(($str.to_string()))
    };
}

pub const LINK_TO_GITHUB: &str = "https://github.com/Optimunn/GeoGame";
pub const LINK_TO_RUST: &str = "https://www.rust-lang.org";    
pub const LINK_TO_SLINT: &str = "https://slint.dev";