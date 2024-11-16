use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type Color = [u8; 3];
pub type Bpm = u8;

// #[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct TrackerConfig {
    pub font: FontConfig,
    pub colors: ColorsConfig,
    pub ui: UiConfig,
}

// #[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct FontConfig {
    pub name: String,
    pub file_path: PathBuf,
    pub size: Vec<usize>,
}

// #[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct ColorsConfig {
    pub back_ground: Color,
    pub hight_light: Color,
    pub text: Color,
    pub text_alt: Color,
    pub border: Color,
    pub cursor: Color,
    pub note_held: Color,
}

// #[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Copy, PartialEq)]
pub struct UiConfig {
    pub header: f64,
    // pub n_cols: usize,
    pub menu: MenuUiConf,
    pub tab: TabUiConf,
}

// #[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Copy, PartialEq)]
pub struct MenuUiConf {
    pub width: f64,
    pub tempo: f64,
    pub note_display: f64,
    // pub eq: f64,
    pub osciloscope: f64,
    pub menu_map: f64,
}

// #[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Copy, PartialEq)]
pub struct TabUiConf {
    pub width: f64,
    pub height: f64,
    pub row_elm_width: f64,
    pub row_height: f64,
}

// #[pyfunction]
pub fn get_config() -> TrackerConfig {
    let mut config = TrackerConfig::default();
    // config.colors.text = [10, 100, 20];
    config.colors.text = [166, 227, 161];
    config.colors.back_ground = [30, 30, 46];
    config.colors.cursor = [137, 180, 250];
    config.ui.menu.tempo = 1.0 / 6.0;
    config.ui.menu.note_display = 2.0 / 6.0;
    config.font.size = vec![30];
    config.ui.menu.osciloscope = 4.0 / 6.0;
    config.ui.menu.menu_map = 1.0;
    config.ui.menu.width = 1.0 / 3.125;
    config.ui.tab.width = 2.0 / 3.0;
    config.ui.tab.height = 1.0;
    config.ui.tab.row_elm_width = 1.0 / 5.0;
    config.ui.tab.row_height = 1.0 / 18.0;

    config
}
