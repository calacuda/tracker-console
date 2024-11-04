use bevy::prelude::Component;
use pyo3::pyclass;

pub type Point = [f64; 2];
pub type RectSize = [f64; 2];
pub type Color = [u8; 3];

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug)]
pub enum RenderCMD {
    Line {
        from: Point,
        to: Point,
        fill_color: Color,
        width: u8,
    },
    Text {
        ancor: Point,
        color: Color,
        text: String,
        font_size: usize,
        // font_name: String,
        /// if false the ancor is the top left
        center: bool,
    },
    Rect {
        ancor: Point,
        size: RectSize,
        fill_color: Color,
        /// if false the ancor is the top left
        center: bool,
    },
    Circle {
        ancor: Point,
        fill_color: Color,
        rad: f64,
    },
    Clear(),
}

#[pyclass(module = "tracker_backend", eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum Button {
    A,
    B,
    X,
    Y,
    Start,
    Select,
    LBump,
    RBump,
    LTrig,
    RTrig,
    Up,
    Down,
    Left,
    Right,
    Menu,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq)]
pub enum InputCMD {
    /// Tells the executor to exit
    Exit(),
    ButtonPress(Button),
    ButtonRelease(Button),
}
