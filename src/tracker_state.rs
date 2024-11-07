use crate::{
    ipc::RustIPC,
    pygame_coms::{Chains, DisplayCursor, Instruments, Phrases, PlaybackCursor, Screen},
};
use bevy::prelude::*;
use log::{debug, info};
use tracker_lib::Tempo;

pub struct TrackerStatePlugin;

#[derive(Debug, Resource)]
pub struct StateUpdated(pub bool);

impl Plugin for TrackerStatePlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::base_display::BaseDisplayPlugin loaded");

        app.insert_resource(StateUpdated(true))
            .insert_resource(Tempo(120))
            .insert_resource(Screen::Song())
            .insert_resource(AllInstruments::default())
            .insert_resource(AllPhrases::default())
            .insert_resource(AllChains::default())
            .insert_resource(PlaybackCursor::NotPlaying())
            .insert_resource(DisplayCursor::default())
            .add_systems(Update, send_state);
    }
}

#[derive(Debug, Resource)]
pub struct AllPhrases(pub Phrases);

impl Default for AllPhrases {
    fn default() -> Self {
        Self([None; 256])
    }
}

#[derive(Debug, Resource)]
pub struct AllChains(pub Chains);

impl Default for AllChains {
    fn default() -> Self {
        Self([None; 256])
    }
}

#[derive(Debug, Resource)]
pub struct AllInstruments(pub Instruments);

impl Default for AllInstruments {
    fn default() -> Self {
        let insts: Instruments = Vec::with_capacity(256);

        // TODO: add default instruments
        // insts[0] =

        Self(insts)
    }
}

fn send_state(coms: ResMut<RustIPC>, mut updated: ResMut<StateUpdated>) {
    if updated.0 {
        // TODO: build a state struct
        info!("sending state to frontend");
        // TODO: send the state struct

        updated.0 = false;
    }
}
