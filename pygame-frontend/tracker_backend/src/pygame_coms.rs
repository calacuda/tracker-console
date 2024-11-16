use crate::config::ui::Bpm;
use bevy::prelude::{Component, Resource};
use pyo3::pyclass;

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

/// a MIDI Note
pub type Note = u8;
/// a collection of all the known phrases.
pub type Phrases = [Option<Phrase>; 256];
/// an index into a list of all known type T
pub type Index = usize;

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Resource)]
pub struct DisplayCursor {
    pub row: usize,
    pub col: usize,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State {
    pub playing: [Option<Note>; 4],
    pub tempo: Bpm,
    pub display_cursor: DisplayCursor,
}
