use crate::{
    controls::MyGamepad,
    pygame_coms::Screen,
    tracker_state::{AllPhrases, StateUpdated, Tempo},
    PlayingState,
};
use bevy::prelude::*;
use bevy_midi::{
    output::{MidiOutput, MidiOutputConnection},
    MidiMessage,
};
use std::{
    time::{Duration, Instant},
    usize,
};

#[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct SyncPulse {
    last_pulse_time: Instant,
    n_pulses: usize,
}

#[derive(Resource, Clone, Debug, Eq, PartialEq)]
pub struct SyncTimer(Timer);

#[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct PlayingPhrase(usize, usize, Option<usize>); // phrase index, step index,

#[derive(Resource, Clone, Debug, Eq, PartialEq)]
pub struct ControllerName(String);

#[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct LastPlayedPulse(Option<usize>);

pub struct MidiOutPlugin;

impl Plugin for MidiOutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SyncPulse {
            last_pulse_time: Instant::now(),
            n_pulses: 0,
        })
        .insert_resource(ControllerName("MPK mini Plus MIDI".into()))
        .insert_resource(LastPlayedPulse(None))
        .insert_resource(SyncTimer(Timer::new(
            Duration::from_secs_f64(60.0 / 120.0 / 24.0),
            TimerMode::Repeating,
        )))
        .insert_resource(PlayingPhrase(0, 0, None))
        .add_systems(
            Update,
            start_playing.run_if(not(in_state(PlayingState::Playing))),
        )
        .add_systems(Update, stop_playing.run_if(in_state(PlayingState::Playing)))
        .add_systems(OnEnter(PlayingState::Playing), setup)
        .add_systems(
            Update,
            sync.run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            Update,
            send_notes
                .run_if(in_state(PlayingState::Playing))
                .run_if(on_sixteenth_note)
                .run_if(not_played_yet),
        )
        // .add_systems(
        //     Update,
        //     (refresh_ports, connect).run_if(in_state(PlayingState::Playing)),
        // )
        ;
    }
}

fn setup(
    tempo: Res<Tempo>,
    screen: Res<Screen>,
    mut next_state: ResMut<NextState<PlayingState>>,
    mut sync_timer: ResMut<SyncTimer>,
    mut playing_phrase: ResMut<PlayingPhrase>,
    output: Res<MidiOutput>,
    connection: Res<MidiOutputConnection>,
    controller: Res<ControllerName>,
) {
    sync_timer.0 = Timer::new(
        Duration::from_secs_f64(60.0 / tempo.0 as f64 / 24.0),
        TimerMode::Repeating,
    );

    // set playback cursor loc.
    match *screen {
        Screen::EditPhrase(phrase_n) => *playing_phrase = PlayingPhrase(phrase_n, 0, None),
        _ => {
            error!("can only play Phrases at this point in time.");

            next_state.set(PlayingState::NotPlaying);
        }
    }

    refresh_ports(&output);
    connect(output, connection, controller);
}

// fn refresh_ports(output: Res<MidiOutput>) {
fn refresh_ports(output: &Res<MidiOutput>) {
    // if input.just_pressed(KeyCode::KeyR) {
    output.refresh_ports();
    // }
}

fn connect(
    output: Res<MidiOutput>,
    connection: Res<MidiOutputConnection>,
    controller: Res<ControllerName>,
    // out_set: Res<MidiOutputSettings>,
) {
    // for (keycode, index) in &KEY_PORT_MAP {
    // if input.just_pressed(*keycode) {
    if !connection.is_connected() {
        let mut ports = output.ports().iter();

        while let Some((name, port)) = ports.next() {
            if name.contains(&controller.0) {
                info!("connecting to {name}");
                output.connect(port.clone());
            }
        }

        show_ports(output);
    }
    // }
    // }
}

fn show_ports(output: Res<MidiOutput>) {
    if output.is_changed() {
        // let text_section = &mut instructions.single_mut().sections[1];
        info!("Available output ports:");
        for (i, (name, _)) in output.ports().iter().enumerate() {
            info!("Port {:?}: {:?}", i, name);
        }
    }
}

// fn show_connection(
//     connection: Res<MidiOutputConnection>,
//     // mut instructions: Query<&mut Text, With<Instructions>>,
// ) {
//     if connection.is_changed() {
//         // let text_section = &mut instructions.single_mut().sections[2];
//         if connection.is_connected() {
//             text_section.value = "Connected".to_string();
//             text_section.style.color = GREEN.into();
//         } else {
//             text_section.value = "Disconnected".to_string();
//             text_section.style.color = RED.into();
//         }
//     }
// }

fn sync(
    mut sync_timer: ResMut<SyncTimer>,
    time: Res<Time>,
    tempo: Res<Tempo>,
    mut pulse: ResMut<SyncPulse>,
    // mut state_updated: EventWriter<StateUpdated>,
) {
    sync_timer.0.tick(time.delta());

    if sync_timer.0.just_finished() {
        // TODO: send sync message

        // warn!("sync");

        pulse.n_pulses += 1;
        pulse.n_pulses %= usize::MAX;

        // set last sync pulse time

        sync_timer
            .0
            .set_duration(Duration::from_secs_f64(60.0 / tempo.0 as f64 / 24.0));
        // sync_timer.0.reset();
    }
}

fn on_sixteenth_note(pulse: Res<SyncPulse>) -> bool {
    // info!("n_pulses {}", pulse.n_pulses);
    // 6 because 24 beats is a quarter note.
    pulse.n_pulses % 6 == 0
}

fn not_played_yet(last_played: Res<LastPlayedPulse>, pulse: Res<SyncPulse>) -> bool {
    // info!(
    //     "n_pulses: {}, last_played: {}",
    //     pulse.n_pulses, last_played.0
    // );

    if let Some(lp) = last_played.0 {
        info!("n_pulses: {}, last_played: {}", pulse.n_pulses, lp);

        pulse.n_pulses > lp
    } else {
        true
    }
}

fn send_notes(
    output: Res<MidiOutput>,
    mut playing: ResMut<PlayingPhrase>,
    phrases: Res<AllPhrases>,
    mut state_updated: EventWriter<StateUpdated>,
    mut last_played: ResMut<LastPlayedPulse>,
    pulse: Res<SyncPulse>,
) {
    if let Some(Some(ref phrase)) = phrases.0.get(playing.0) {
        if let Some(last_row) = playing.2 {
            // get note offs from last step
            if let Some(note) = phrase.rows[last_row].note {
                // TODO: make the note a collection of midi Commands
                let channel = phrase.rows[last_row].instrument.unwrap_or(0) % 16;
                let cmd = 0b1000_0000 + (channel as u8);

                // send note offs
                output.send(MidiMessage {
                    msg: [cmd, note, 127].into(),
                });
                // info!("Stopping Note: {note}");
                // state_updated.send_default();
            }
        }

        // get notes to turn on
        let row = playing.1;

        // send note ons
        if let Some(note) = phrase.rows[row].note {
            let channel = phrase.rows[row].instrument.unwrap_or(0) % 16;
            let cmd = 0b1001_0000 + (channel as u8);

            output.send(MidiMessage {
                msg: [cmd, note, 127].into(),
            });
            // info!("Playing Note: {note}");
            // state_updated.send_default();
        }

        // setup for next beat
        // if playing.2.is_some() {
        playing.1 += 1;
        playing.1 %= 16;
        // }

        match &mut playing.2 {
            Some(ref mut val) => *val = row,
            None => playing.2 = Some(0),
        };

        // warn!("playing.1 = {}, playing.2 = {:?}", playing.1, playing.2);
        state_updated.send_default();
    }

    _ = last_played.0.insert(pulse.n_pulses);
}

fn start_playing(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    gamepads: Res<Gamepads>,
    mut playing_state: ResMut<NextState<PlayingState>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let select_button = if let Some(name) = gamepads.name(gamepad)
        && name.starts_with("PS5")
    {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Select,
        }
    } else {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }
    };

    if buttons.just_released(select_button)
        && !buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Mode,
        })
    {
        playing_state.set(PlayingState::Playing);
    }
}

fn stop_playing(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    gamepads: Res<Gamepads>,
    mut playing_state: ResMut<NextState<PlayingState>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let select_button = if let Some(name) = gamepads.name(gamepad)
        && name.starts_with("PS5")
    {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Select,
        }
    } else {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }
    };

    if buttons.just_released(select_button) {
        playing_state.set(PlayingState::NotPlaying);
    }
}
