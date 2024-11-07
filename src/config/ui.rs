use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

pub type Color = [u8; 3];

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct TrackerConfig {
    pub font: FontConfig,
    pub colors: ColorsConfig,
    pub ui: UiConfig,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Eq, Hash, PartialEq)]
pub struct FontConfig {
    pub name: String,
    pub file_path: PathBuf,
    pub size: Arc<[usize]>,
}

#[pyclass(module = "tracker_backend", get_all)]
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

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Copy, PartialEq)]
pub struct UiConfig {
    pub header: f64,
    pub n_cols: usize,
    pub menu: MenuUiConf,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, Copy, PartialEq)]
pub struct MenuUiConf {
    pub width: f64,
    pub tempo: f64,
    pub note_display: f64,
    // pub eq: f64,
    pub osciloscope: f64,
    pub menu_map: f64,
}
