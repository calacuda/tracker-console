#![feature(let_chains, impl_trait_in_assoc_type)]
use bevy::prelude::*;
use graph::GraphStatePlugin;
use stepper::Pattern;
use tracker_state::TrackerStatePlugin;

pub mod config;
pub mod controls;
pub mod graph;
pub mod stepper;
pub mod tracker_state;

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

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(ScreenState = ScreenState::Graph)]
pub enum GraphSubState {
    /// used when adding a new node
    NewNode,
    /// moving an already placed node.
    MoveNode,
    /// editing the parameters of an already placed node.
    EditNode,
    // /// edits a nodes arguments
    // EditArgs,
    #[default]
    Neuteral,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(ScreenState = ScreenState::Stepper)]
pub enum StepperChannel {
    #[default]
    Channel1,
    Channel2,
    Channel3,
    Channel4,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(ScreenState = ScreenState::Stepper)]
pub enum StepperChannelParam {
    #[default]
    Channels,
    Params,
}

// #[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
// #[source(ScreenState = ScreenState::Stepper)]
// pub enum StepperChannelParam {}

pub type MidiNote = u8;
/// an index into the phrases list
pub type PatternIndex = usize;
pub type AllPatterns = [Option<Pattern>; GRAPH_Y * GRAPH_X];
pub type InstrumentIndex = usize;

pub const GRAPH_X: usize = 256;
pub const GRAPH_Y: usize = 256;

pub struct TrackerCorePlugin;

impl Plugin for TrackerCorePlugin {
    fn build(&self, app: &mut App) {
        info!("loading TrackerCorePlugin");

        app.add_plugins(GraphStatePlugin)
            .add_plugins(TrackerStatePlugin)
            .init_state::<ScreenState>()
            .add_sub_state::<GraphSubState>()
            .add_sub_state::<StepperChannel>()
            .add_sub_state::<StepperChannelParam>()
            .init_state::<PlayingState>()
            .init_state::<ExitMenuState>();
    }
}
