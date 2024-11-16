#![feature(let_chains)]
use bevy::prelude::*;
use stepper::Pattern;

pub mod graph;
pub mod stepper;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenState {
    #[default]
    Graph,
    Stepper,
    PlaySynth,
    Settings,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayingState {
    Playing,
    #[default]
    NotPlaying,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExitMenuState {
    Opened,
    #[default]
    Closed,
}

// pub mod chain_menu;
pub mod config;
pub mod controls;
pub mod tracker_state;

pub type MidiNote = u8;
/// an index into the phrases list
pub type PatternIndex = usize;
pub type Patterns = [Option<Pattern>; 256];
