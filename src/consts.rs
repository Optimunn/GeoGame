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

pub mod pallet {
    use slint::Color;

    #[allow(dead_code)]
    pub const COLOR_RED: Color = Color::from_argb_u8(255, 255, 0, 0);
    #[allow(dead_code)]
    pub const COLOR_GREEN: Color = Color::from_argb_u8(255, 0, 255, 0);
    #[allow(dead_code)]
    pub const COLOR_GRAY: Color = Color::from_argb_u8(255, 128, 128, 128);
    #[allow(dead_code)]
    pub const COLOR_FREEDOM: Color = Color::from_argb_encoded(0xffe786d2);
    #[allow(dead_code)]
    pub const COLOR_LAVENDER: Color = Color::from_argb_encoded(0xffab86e7);
    #[allow(dead_code)]
    pub const COLOR_BLUE_SKY: Color = Color::from_argb_encoded(0xff5f79ef);
    #[allow(dead_code)]
    pub const COLOR_MANDARIN: Color = Color::from_argb_encoded(0xffefbd5f);
    #[allow(dead_code)]
    pub const COLOR_RIPE_LIME: Color = Color::from_argb_encoded(0xff45f931);
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

#[macro_export]
macro_rules! drop_buf {
    ($str:expr) => {
        &PathBuf::from(($str.to_string()))
    };
}

pub const LINK_TO_GITHUB: &str = "https://github.com/Optimunn/GeoGame";
pub const LINK_TO_RUST: &str = "https://www.rust-lang.org";
pub const LINK_TO_SLINT: &str = "https://slint.dev";