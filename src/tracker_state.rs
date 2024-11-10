use crate::{
    config::ui::Bpm,
    ipc::RustIPC,
    pygame_coms::{
        Chains, DisplayCursor, Instruments, Phrases, PlaybackCursor, PlaybackCursorWrapper, Screen,
        ScreenData, Song, State,
    },
    ScreenState,
};
use bevy::{log::*, prelude::*};

pub struct TrackerStatePlugin;

#[derive(Debug, Event, Default)]
pub struct StateUpdated;

impl Plugin for TrackerStatePlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::base_display::BaseDisplayPlugin loaded");

        app
            // .insert_resource(StateUpdated(true))
            .add_event::<StateUpdated>()
            .insert_resource(Tempo(120))
            .insert_resource(Screen::Song())
            .insert_resource(AllInstruments::default())
            .insert_resource(AllPhrases::default())
            .insert_resource(AllChains::default())
            .insert_resource(PlaybackCursorWrapper::default())
            .insert_resource(DisplayCursor::default())
            .insert_resource(Song::default())
            .add_systems(Update, update_state)
            .add_systems(OnEnter(ScreenState::EditSong), send_state)
            .add_systems(OnEnter(ScreenState::EditChain), send_state)
            .add_systems(OnEnter(ScreenState::EditPhrase), send_state)
            .add_systems(OnEnter(ScreenState::EditInsts), send_state)
            .add_systems(OnEnter(ScreenState::PlaySynth), send_state)
            .add_systems(OnEnter(ScreenState::Settings), send_state);
    }
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Resource)]
pub struct Tempo(pub Bpm);

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
        // let mut s = Self([None; 256]);
        // s.0[0] = Some(Chain::default());
        //
        // s
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

// fn run_if_state_updated(updated: Res<StateUpdated>) -> bool {
//     updated.0
// }
fn send_state(mut state_update_events: EventWriter<StateUpdated>) {
    state_update_events.send_default();
}

fn update_state(
    coms: ResMut<RustIPC>,
    mut state_update_events: EventReader<StateUpdated>,
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
    for _ev in state_update_events.read() {
        let screen = match *screen {
            Screen::Song() => ScreenData::Song(song.clone()),
            Screen::Settings() => ScreenData::Settings(),
            Screen::EditChain(i) => ScreenData::Chain(chains.0[i].unwrap()),
            Screen::EditPhrase(i) => ScreenData::Phrase(phrases.0[i].unwrap()),
            Screen::Instrument(i) => ScreenData::Instrument(instruments.0[i].clone().unwrap()),
            Screen::PlaySynth() => ScreenData::PlaySynth(),
        };

        let playing = match playing.0.lock().unwrap().clone() {
            PlaybackCursor::NotPlaying() => [None, None, None, None],
            PlaybackCursor::NotFull {
                from_screen: _,
                row: _,
            } => [None, None, None, None],
            PlaybackCursor::FullSong {
                lead_1: _,
                lead_2: _,
                bass: _,
                perc: _,
                row: _,
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
            info!("Sent state to Python");
        }

        info!("sending complete");
    }
}
