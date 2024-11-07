use std::ops::Deref;

use crate::{
    ipc::RustIPC,
    pygame_coms::{
        Chains, DisplayCursor, Instruments, Phrases, PlaybackCursor, PlaybackCursorWrapper, Screen,
        ScreenData, Song, State,
    },
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
            .insert_resource(PlaybackCursorWrapper::default())
            .insert_resource(DisplayCursor::default())
            .insert_resource(Song::default())
            .add_systems(Update, send_state.run_if(run_if_state_updated));
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

fn send_state(
    coms: ResMut<RustIPC>,
    mut updated: ResMut<StateUpdated>,
    tempo: Res<Tempo>,
    screen: Res<Screen>,
    instruments: Res<AllInstruments>,
    phrases: Res<AllPhrases>,
    chains: Res<AllChains>,
    playing: Res<PlaybackCursorWrapper>,
    display_cursor: Res<DisplayCursor>,
    song: Res<Song>,
    // playing: Res<PlaybackCursor>,
) {
    // if updated.0 {
    // build a state struct
    // let state = State {
    //     chains: chains.0.clone(),
    //     phrases: phrases.0.clone(),
    //     instruments: instruments.0.clone(),
    //     display_cursor: display_cursor.clone(),
    //     screen: screen.clone(),
    //     tempo: tempo.0,
    //     song: song.clone(),
    //     playing: playing.0.lock().unwrap().clone(),
    // };
    let screen = match *screen {
        Screen::Song() => ScreenData::Song(song.clone()),
        Screen::Settings() => ScreenData::Settings(),
        Screen::EditChain(i) => ScreenData::Chain(chains.0[i].unwrap()),
        Screen::EditPhrase(i) => ScreenData::Phrase(phrases.0[i].unwrap()),
        Screen::Instrument(i) => ScreenData::Instrument(instruments.0[i].clone()),
        Screen::PlaySynth() => ScreenData::PlaySynth(),
    };

    let playing = match playing.0.lock().unwrap().clone() {
        PlaybackCursor::NotPlaying() => [None, None, None, None],
        PlaybackCursor::NotFull { from_screen, row } => [None, None, None, None],
        PlaybackCursor::FullSong {
            lead_1,
            lead_2,
            bass,
            perc,
            row,
        } => {
            // TODO: do the thing
            [None, None, None, None]
        }
    };

    let state = State {
        display_cursor: display_cursor.clone(),
        screen,
        tempo: tempo.0,
        song: song.clone(),
        playing,
    };

    info!("sending state to frontend");
    // send the state struct
    if let Err(e) = coms.send_msg(state) {
        error!("sending updated state failed with error: {e}");
    } else {
        updated.0 = false;
    }

    info!("sending complete");
    // }
}

fn run_if_state_updated(updated: Res<StateUpdated>) -> bool {
    updated.0
}
