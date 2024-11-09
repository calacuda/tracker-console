use crate::{
    pygame_coms::{DisplayCursor, Index, Screen, Song},
    tracker_state::{AllChains, AllInstruments, AllPhrases},
    ScreenState,
};
use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    log::*,
    prelude::*,
};
use std::ops::Deref;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::controls::ControlsPlugin loaded");

        app.insert_resource(LastViewed::default())
            .insert_resource(LastAdded::default())
            .init_resource::<NextScreen>()
            .add_systems(Update, gamepad_connections)
            .add_systems(Update, screen_change);
        // .add_systems(Update, gamepad_input);
    }
}

#[derive(Debug, Resource, Default)]
pub struct LastViewed {
    pub chain: Index,
    pub phrase: Index,
    pub instrument: Index,
}

#[derive(Debug, Resource, Default)]
pub struct LastAdded {
    pub chain: Index,
    pub phrase: Index,
    pub instrument: Index,
    pub note: Index,
    pub command: Index,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub struct NextScreen(Option<ScreenState>);

/// Simple resource to store the ID of the first connected gamepad.
/// We can use it to know which gamepad to use for player input.
#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}

fn update_state(
    screen: Screen,
    // mut next_screen: ResMut<NextState<ScreenState>>,
    // mut screen_will_be: ResMut<NextScreen>,
    instruments: Res<AllInstruments>,
    phrases: Res<AllPhrases>,
    chains: Res<AllChains>,
    song: Res<Song>,
    display_cursor: Res<DisplayCursor>,
    mut last_viewed: ResMut<LastViewed>,
    n_s: ScreenState,
) -> Option<(Screen, ScreenState)> {
    // if let Some(n_s) = screen_will_be.0 {
    match (screen.clone(), n_s) {
        // edit_song -> settings
        (Screen::Song(), ScreenState::Settings) => {
            // *screen = Screen::Settings();
            Some((Screen::Settings(), ScreenState::Settings))
        }
        // edit_song -> edit_chain
        (Screen::Song(), ScreenState::EditChain) => {
            let chain = song.rows[display_cursor.row][display_cursor.col];

            // warn!("{display_cursor:?} => {chain:?}");

            if let Some(chain_i) = chain {
                // *screen = Screen::EditChain(chain);
                Some((Screen::EditChain(chain_i), ScreenState::EditChain))
            } else if chains.deref().0[last_viewed.chain].is_some() {
                // *screen = Screen::EditChain(last_viewed.chain);
                Some((Screen::EditChain(last_viewed.chain), ScreenState::EditChain))
            } else {
                warn!("not shifting to chain as no chains are available");
                None
            }
        }
        // edit_chains -> edit_phrase
        (Screen::EditChain(chain_i), ScreenState::EditPhrase) => {
            let phrase = chains.deref().0[chain_i].unwrap().rows[display_cursor.row].phrase;

            if let Some(phrase_i) = phrase {
                last_viewed.chain = chain_i;
                Some((Screen::EditPhrase(phrase_i), ScreenState::EditPhrase))
            } else if phrases.0[last_viewed.phrase].is_some() {
                // *screen = Screen::EditChain(last_viewed.chain);
                Some((
                    Screen::EditChain(last_viewed.phrase),
                    ScreenState::EditChain,
                ))
            } else {
                None
            }
        }
        // edit_chains -> Song
        (Screen::EditChain(chain_i), ScreenState::EditSong) => {
            last_viewed.chain = chain_i;
            Some((Screen::Song(), ScreenState::EditSong))
        }
        // edit_phrase -> edit_chains
        (Screen::EditPhrase(phrase_i), ScreenState::EditChain) => {
            if phrases.0.get(last_viewed.phrase).is_some() {
                last_viewed.phrase = phrase_i;
                Some((
                    Screen::EditChain(last_viewed.chain),
                    ScreenState::EditPhrase,
                ))
            } else {
                None
            }
        }
        // edit_phrase -> edit_instrument
        (Screen::EditPhrase(phrase_i), ScreenState::EditInsts) => {
            let instrument =
                phrases.deref().0[phrase_i].unwrap().rows[display_cursor.row].instrument;

            if let Some(instrument_i) = instrument {
                // *screen = Screen::Instrument(instrument_i);
                last_viewed.phrase = phrase_i;
                Some((Screen::Instrument(instrument_i), ScreenState::EditInsts))
            } else if instruments.0[last_viewed.instrument % 256].is_some() {
                // *screen = Screen::Instrument(last_viewed.instrument % instruments.0.len());
                last_viewed.phrase = phrase_i;
                Some((
                    Screen::Instrument(last_viewed.instrument % 256),
                    ScreenState::EditInsts,
                ))
            } else {
                None
            }
        }
        // edit_instrument -> edit_phrase
        (Screen::Instrument(inst_i), ScreenState::EditPhrase) => {
            // *screen = Screen::EditPhrase(last_viewed.phrase);
            if chains.0.get(last_viewed.chain).is_some() {
                last_viewed.instrument = inst_i;
                Some((
                    Screen::EditPhrase(last_viewed.phrase),
                    ScreenState::EditPhrase,
                ))
            } else {
                None
            }
        }
        // edit_instrument -> play_synth
        (Screen::Instrument(inst_i), ScreenState::PlaySynth) => {
            // *screen = Screen::PlaySynth();
            last_viewed.instrument = inst_i;
            Some((Screen::PlaySynth(), ScreenState::PlaySynth))
        }
        // play_synth -> edit_instrument
        (Screen::PlaySynth(), ScreenState::EditInsts) => {
            // *screen = Screen::Instrument(last_viewed.instrument);
            if instruments.0.get(last_viewed.instrument).is_some() {
                Some((
                    Screen::Instrument(last_viewed.instrument),
                    ScreenState::EditInsts,
                ))
            } else {
                None
            }
        }
        // play_synth -> settings
        (Screen::PlaySynth(), ScreenState::Settings) => {
            // *screen = Screen::Settings();
            Some((Screen::Settings(), ScreenState::Settings))
        }
        // settings -> play_synth
        (Screen::Settings(), ScreenState::PlaySynth) => {
            // *screen = Screen::PlaySynth();
            Some((Screen::PlaySynth(), ScreenState::PlaySynth))
        }
        // settings -> edit_song
        (Screen::Settings(), ScreenState::EditSong) => {
            // *screen = Screen::Song();
            Some((Screen::Song(), ScreenState::EditSong))
        }
        (Screen::Song(), ScreenState::EditSong)
        | (Screen::EditChain(_), ScreenState::EditChain)
        | (Screen::EditPhrase(_), ScreenState::EditPhrase)
        | (Screen::Instrument(_), ScreenState::EditInsts)
        | (Screen::PlaySynth(), ScreenState::PlaySynth)
        | (Screen::Settings(), ScreenState::Settings) => None,
        (from, to) => {
            error!("transisioning from tab: {from:?} to tab: {to:?}, is illegal");
            None
        }
    }
}

fn screen_change(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    state: ResMut<State<ScreenState>>,
    mut next_screen: ResMut<NextState<ScreenState>>,
    mut screen_res: ResMut<Screen>,
    instruments: Res<AllInstruments>,
    phrases: Res<AllPhrases>,
    chains: Res<AllChains>,
    song: Res<Song>,
    display_cursor: Res<DisplayCursor>,
    last_viewed: ResMut<LastViewed>,
    gamepads: Res<Gamepads>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let left_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };
    let a_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    // if let Some(gp) = gamepads.name(gamepad) {
    //     warn!("game pad name = {gp}");
    // }

    let start_button = if let Some(name) = gamepads.name(gamepad)
        && name.starts_with("PS5")
    {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }
    } else {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Select,
        }
    };

    let screens = [
        ScreenState::EditSong,
        ScreenState::EditChain,
        ScreenState::EditPhrase,
        ScreenState::EditInsts,
        ScreenState::PlaySynth,
        ScreenState::Settings,
    ];

    let Some(screen_i) = screens.into_iter().position(|s| s == **state) else {
        // unhandled screen
        error!("the current screen is {:?}, but the screen_change system is not set up to handle that yet.", **state);
        return;
    };

    if buttons.just_released(left_button)
        && buttons.pressed(start_button)
        && !buttons.pressed(a_button)
    {
        // button just pressed: make the player jump
        info!("menu tab move left");
        let i = if screen_i > 0 {
            (screen_i - 1) % screens.len()
        } else {
            screens.len() - 1
        };

        let change_to = screens[i];
        // (*screen_will_be).0 = Some(screen);
        if let Some((screen, screen_state)) = update_state(
            screen_res.clone(),
            instruments,
            phrases,
            chains,
            song,
            display_cursor,
            last_viewed,
            change_to,
        ) {
            next_screen.set(screen_state);
            *screen_res = screen
        }
    } else if buttons.just_released(right_button)
        && buttons.pressed(start_button)
        && !buttons.pressed(a_button)
    {
        // button just pressed: make the player jump
        info!("menu tab move right");
        let change_to = screens[(screen_i + 1) % screens.len()];
        // (*screen_will_be).0 = Some(screen);
        if let Some((screen, screen_state)) = update_state(
            screen_res.clone(),
            instruments,
            phrases,
            chains,
            song,
            display_cursor,
            last_viewed,
            change_to,
        ) {
            next_screen.set(screen_state);
            *screen_res = screen
        }
    }
}

// fn gamepad_input(
//     buttons: Res<ButtonInput<GamepadButton>>,
//     my_gamepad: Option<Res<MyGamepad>>,
//     // keys: Res<ButtonInput<KeyCode>>,
// ) {
//     let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
//         // no gamepad is connected
//         return;
//     };
//
//     // In a real game, the buttons would be configurable, but here we hardcode them
//     let b_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::South,
//     };
//     let a_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::East,
//     };
//     let x_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::North,
//     };
//     let y_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::West,
//     };
//     let up_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadUp,
//     };
//     let down_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadDown,
//     };
//     let left_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadLeft,
//     };
//     let right_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadRight,
//     };
//
//     // if buttons.just_pressed(b_button) || keys.just_pressed(KeyCode::KeyZ) {
//     if buttons.just_pressed(b_button) {
//         // button just pressed: make the player jump
//         println!("B Button pressed");
//     }
//
//     // if buttons.just_pressed(a_button) || keys.just_pressed(KeyCode::KeyX) {
//     if buttons.just_pressed(a_button) {
//         // button being held down: heal the player
//         println!("A Button pressed");
//     }
//
//     if buttons.just_pressed(x_button) {
//         println!("X Button pressed");
//     }
//
//     if buttons.just_pressed(y_button) {
//         println!("Y Button pressed");
//     }
//
//     if buttons.just_pressed(up_button) {
//         println!("UP Button pressed");
//     }
//
//     if buttons.just_pressed(down_button) {
//         println!("DOWN Button pressed");
//     }
//
//     if buttons.just_pressed(left_button) {
//         println!("LEFT Button pressed");
//     }
//
//     if buttons.just_pressed(right_button) {
//         println!("RIGHT Button pressed");
//     }
// }
