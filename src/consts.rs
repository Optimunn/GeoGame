#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod os {
#[cfg(debug_assertions)]
    pub const LOAD_CONFIG: &str = "save/config.json";
#[cfg(debug_assertions)]
    pub const LOAD_DATA: &str = "data/country_en.json";
#[cfg(debug_assertions)]
    pub const LOAD_IMAGE: &str = "assets/flags/4x3/";
#[cfg(debug_assertions)]
    pub const LOAD_ICON: &str = "assets/icons/";

#[cfg(not(debug_assertions))]
    pub const LOAD_CONFIG: &str = "../Resources/Save/config.json";
#[cfg(not(debug_assertions))]
    pub const LOAD_DATA: &str = "../Resources/Data/country_en.json";
#[cfg(not(debug_assertions))]
    pub const LOAD_IMAGE: &str = "../Resources/Assets/flags/";
#[cfg(not(debug_assertions))]
    pub const LOAD_ICON: &str = "../icons/earth.svg";
}

#[cfg(target_os = "windows")]
pub mod os {
#[cfg(debug_assertions)]
    pub const LOAD_CONFIG: &str = "save\\config.json";
#[cfg(debug_assertions)]
    pub const LOAD_DATA: &str = "data\\country_en.json";
#[cfg(debug_assertions)]
    pub const LOAD_IMAGE: &str = "assets\\flags\\4x3\\";
#[cfg(debug_assertions)]
    pub const LOAD_ICON: &str = "assets\\icons\\";

#[cfg(not(debug_assertions))]
    pub const LOAD_CONFIG: &str = "..\\Resources\\Save\\config.json";
#[cfg(not(debug_assertions))]
    pub const LOAD_DATA: &str = "..\\Resources\\Data\\country_en.json";
#[cfg(not(debug_assertions))]
    pub const LOAD_IMAGE: &str = "..\\Resources\\Assets\\flags\\";
#[cfg(not(debug_assertions))]
    pub const LOAD_ICON: &str = "..\\icons\\earth.svg";
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