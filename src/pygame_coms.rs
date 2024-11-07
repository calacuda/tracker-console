use std::sync::{Arc, Mutex};

use bevy::prelude::{Component, Resource};
use pyo3::pyclass;
use tracker_lib::Bpm;

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

/// a MIDI Note
pub type Note = u8;
/// a collection of all the known phrases.
pub type Phrases = [Option<Phrase>; 256];
/// a collection of all the known chains.
pub type Chains = [Option<Chain>; 256];
/// a collection of all the known instruments.
pub type Instruments = Vec<Instrument>;
/// an index into a list of all known type T
pub type Index = usize;

/// a command used in the a Phrase
#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TrackerCommand {
    Volume(f32),
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Instrument {
    /// true if the instruemnt is a synth, false if its percussion
    pub synth: bool,
    pub human_name: String,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhraseRow {
    pub note: Option<Note>,
    pub instrument: Option<Index>,
    pub command: Option<TrackerCommand>,
}

/// a single phrase to be used as a part of chains
#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Phrase {
    pub rows: [PhraseRow; 16],
    pub name: Index,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChainRow {
    pub phrase: Option<Index>,
}

/// a chain of phrases strung together to make a song
#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Chain {
    pub rows: [ChainRow; 16],
    pub name: Index,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct SongRow {
    pub lead_1: Option<Index>,
    pub lead_2: Option<Index>,
    pub bass: Option<Index>,
    pub perc: Option<Index>,
}

/// the whole song
#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Resource)]
pub struct Song {
    pub rows: [SongRow; 16],
    // pub name: Index,
    pub default_instrument: [Index; 4],
}

impl Default for Song {
    fn default() -> Self {
        Self {
            rows: [SongRow::default(); 16],
            default_instrument: [0, 0, 1, 2],
        }
    }
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Resource)]
pub enum Screen {
    Song(),
    EditChain(Index),
    EditPhrase(Index),
    Instrument(Index),
    PlaySynth(),
    Settings(),
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum PlaybackCursor {
    /// playback of the full song
    FullSong {
        lead_1: Vec<(Screen, Index)>,
        lead_2: Vec<(Screen, Index)>,
        bass: Vec<(Screen, Index)>,
        perc: Vec<(Screen, Index)>,
        row: Index,
    },
    /// playback of either a chain or phrase
    NotFull {
        /// a stack. the first element of it is the start screen. the most recent addition is the
        /// curent screen being played. if on started playing on row 4 of chain 7 this would look like
        /// this: [ (Chain(7), 4), (Phrase(the phrase that Chain(7) row 4 points to), 0) ]
        from_screen: Vec<(Screen, Index)>,
        row: Index,
    },
    NotPlaying(),
}

impl Default for PlaybackCursor {
    fn default() -> Self {
        Self::NotPlaying()
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub struct PlaybackCursorWrapper(pub Arc<Mutex<PlaybackCursor>>);

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Resource)]
pub struct DisplayCursor {
    pub row: usize,
    pub col: usize,
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Resource)]
pub enum ScreenData {
    Song(Song),
    Chain(Chain),
    Phrase(Phrase),
    Instrument(Instrument),
    PlaySynth(),
    Settings(),
}

#[pyclass(module = "tracker_backend", get_all)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State {
    pub song: Song,
    // /// the screen curently being displayed to the user
    // pub screen: Screen,
    // pub phrases: Phrases,
    // pub chains: Chains,
    // pub instruments: Instruments,
    // /// set to none when not playing, set to some value when playing.
    // pub playing: PlaybackCursor,
    pub screen: ScreenData,
    pub playing: [Option<Note>; 4],
    pub tempo: Bpm,
    pub display_cursor: DisplayCursor,
}
