use crate::{
    pygame_coms::Screen,
    tracker_state::{AllPhrases, StateUpdated, Tempo},
    PlayingState,
};
use bevy::prelude::*;
use bevy_midi::{
    output::{MidiOutput, MidiOutputConnection},
    MidiMessage,
};
use midi_msg::{MidiMsg, SystemRealTimeMsg};
use std::{
    time::{Duration, Instant},
    usize,
};

#[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct SyncPulse {
    last_pulse_time: Instant,
    pub n_pulses: usize,
}

#[derive(Resource, Clone, Debug, Eq, PartialEq)]
pub struct SyncTimer(Timer);

#[derive(Component, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct PlayingPhrase(pub usize, pub usize, pub Option<usize>); // phrase index, step index,

#[derive(Resource, Clone, Debug, Eq, PartialEq)]
pub struct ControllerName(String);

#[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct LastPlayedPulse(Option<usize>);

#[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct BPQ(pub usize);

#[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq, Deref, DerefMut)]
pub struct PlayingSyncPulse(pub bool);

// #[derive(Resource, Clone, Debug, Copy, Eq, Hash, PartialEq)]
// pub struct PlayHead

#[derive(Component, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct PlayingQueued;

#[derive(Component, Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub struct QueueStopPlaying;

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
        // .insert_resource(PlayingPhrase(0, 0, None))
        .insert_resource(BPQ(24))
        .insert_resource(PlayingSyncPulse(false))
        .add_systems(
            Update,
            // start_playing.run_if(not(in_state(PlayingState::Playing))),
            toggle_playing,
        )
        .add_systems(Update, connect)
        // .add_systems(Update, (stop_queued, stop_playing).run_if(in_state(PlayingState::Playing)))
        .add_systems(Update, toggle_syncing)
        .add_systems(Update, (play_queued, (stop_queued, stop_playing).chain()).run_if(in_state(PlayingState::Playing)).run_if(should_play_queue))
        .add_systems(OnEnter(PlayingState::Playing), setup)
        .add_systems(OnExit(PlayingState::Playing), cleanup)
        .add_systems(
            Update,
            sync.run_if(sync_pulsing),
        )
        .add_systems(
            Update,
            (
                send_notes.run_if(in_state(PlayingState::Playing)),
                update_front_end.run_if(sync_pulsing)
            )
            .chain()
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
    // mut cmds: Commands,
    tempo: Res<Tempo>,
    screen: Res<Screen>,
    mut next_state: ResMut<NextState<PlayingState>>,
    mut sync_timer: ResMut<SyncTimer>,
    // mut playing_phrase: ResMut<PlayingPhrase>,
    output: Res<MidiOutput>,
    connection: Res<MidiOutputConnection>,
    controller: Res<ControllerName>,
    bpq: Res<BPQ>,
) {
    sync_timer.0 = Timer::new(
        Duration::from_secs_f64(60.0 / tempo.0 as f64 / bpq.0 as f64),
        TimerMode::Repeating,
    );

    // set playback cursor loc.
    match *screen {
        Screen::EditPhrase(_phrase_n) => {
            // cmds.spawn(PlayingPhrase(phrase_n, 0, None));
        }
        _ => {
            error!("can only play Phrases at this point in time.");

            next_state.set(PlayingState::NotPlaying);
        }
    }

    connect(output, connection, controller);
}

fn cleanup(
    mut cmds: Commands,
    mut playing_phrases: Query<(Entity, &mut PlayingPhrase)>,
    mut state_updated: EventWriter<StateUpdated>,
) {
    for (id, ref mut playing_phrase) in playing_phrases.iter_mut() {
        // set playback cursor loc.
        playing_phrase.2 = None;
        playing_phrase.1 = 0;
        cmds.entity(id).insert(PlayingQueued);
        state_updated.write_default();
    }
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
        refresh_ports(&output);

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

fn sync_pulsing(pulsing: Res<PlayingSyncPulse>) -> bool {
    **pulsing
}

fn sync(
    mut sync_timer: ResMut<SyncTimer>,
    time: Res<Time>,
    tempo: Res<Tempo>,
    mut pulse: ResMut<SyncPulse>,
    // mut state_updated: EventWriter<StateUpdated>,
    output: Res<MidiOutput>,
    bpq: Res<BPQ>,
) {
    sync_timer.0.tick(time.delta());

    if sync_timer.0.just_finished() {
        // if pulse.n_pulses == 0 {
        //     let midi_bytes = MidiMsg::SystemRealTime {
        //         msg: SystemRealTimeMsg::Start,
        //     }
        //     .to_midi();
        //
        //     output.send(MidiMessage {
        //         msg: midi_bytes.into(),
        //     });
        // }

        // send sync message
        let midi_bytes = MidiMsg::SystemRealTime {
            msg: SystemRealTimeMsg::TimingClock,
        }
        .to_midi();

        output.send(MidiMessage {
            msg: midi_bytes.into(),
        });

        // warn!("sync");

        pulse.n_pulses += 1;
        pulse.n_pulses %= usize::MAX;

        // set last sync pulse time

        sync_timer.0.set_duration(Duration::from_secs_f64(
            60.0 / tempo.0 as f64 / bpq.0 as f64,
        ));
        // sync_timer.0.reset();
    }
}

fn on_sixteenth_note(pulse: Res<SyncPulse>, bpq: Res<BPQ>) -> bool {
    // info!("n_pulses {}", pulse.n_pulses);
    // 6 because 24 beats is a quarter note.
    pulse.n_pulses % (bpq.0 / 4) == 0
}

fn not_played_yet(last_played: Res<LastPlayedPulse>, pulse: Res<SyncPulse>) -> bool {
    // info!(
    //     "n_pulses: {}, last_played: {}",
    //     pulse.n_pulses, last_played.0
    // );

    if let Some(lp) = last_played.0 {
        debug!("n_pulses: {}, last_played: {}", pulse.n_pulses, lp);

        pulse.n_pulses > lp
    } else {
        true
    }
}

fn update_front_end(mut state_updated: EventWriter<StateUpdated>) {
    state_updated.write_default();
}

fn send_notes(
    output: Res<MidiOutput>,
    mut playing: Query<&mut PlayingPhrase, Without<PlayingQueued>>,
    phrases: Res<AllPhrases>,
    // mut state_updated: EventWriter<StateUpdated>,
    mut last_played: ResMut<LastPlayedPulse>,
    pulse: Res<SyncPulse>,
) {
    for ref mut playing in playing.iter_mut() {
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
                // error!("Playing Note: {note}");
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
            // state_updated.write_default();
        }
    }

    _ = last_played.0.insert(pulse.n_pulses);
}

fn toggle_playing(
    mut cmds: Commands,
    buttons: Single<&Gamepad>,
    mut playing_state: ResMut<NextState<PlayingState>>,
    current_play_state: Res<State<PlayingState>>,
    // mut playing_sync: ResMut<PlayingSyncPulse>,
    playing: Query<(Entity, &PlayingPhrase)>,
    screen: Res<Screen>,
) {
    let start_button = GamepadButton::Start;

    if buttons.just_released(start_button) && !buttons.pressed(GamepadButton::Mode) {
        match *screen {
            Screen::EditPhrase(phrase_n) => {
                if *current_play_state.get() != PlayingState::Playing {
                    playing_state.set(PlayingState::Playing);
                }
                // else if *current_play_state.get() != PlayingState::Playing {
                //     playing_state.set(PlayingState::NotPlaying);
                // }

                // playing_sync.0 = true;
                // info!("starting sync pulse");

                let maybe_playing = playing
                    .iter()
                    .find(|(_entity, playing)| playing.0 == phrase_n);

                if let Some((already_playing, _)) = maybe_playing {
                    info!("stop playback event queued for: {phrase_n}");
                    cmds.entity(already_playing).insert(QueueStopPlaying);
                } else {
                    info!("queuing playing for: {phrase_n}");
                    cmds.spawn((PlayingPhrase(phrase_n, 0, None), PlayingQueued));
                }
            }
            _ => {}
        };
    }
}

fn should_play_queue(pulse: Res<SyncPulse>, bpq: Res<BPQ>) -> bool {
    let to_play = ((pulse.n_pulses / (bpq.0 / 4)) % 16) == 0;

    // info!("to_play = {to_play}, {}", pulse.n_pulses);

    to_play
}

fn play_queued(
    mut cmds: Commands,
    playing_queue: Query<(Entity, &PlayingPhrase), With<PlayingQueued>>,
) {
    for (id, phrase) in playing_queue {
        info!("playing queued phrase: {}", phrase.0);

        cmds.entity(id).remove::<PlayingQueued>();
    }
}

fn stop_queued(
    mut cmds: Commands,
    stop_queue: Query<(Entity, &PlayingPhrase), With<QueueStopPlaying>>,
) {
    for (id, phrase) in stop_queue {
        info!("stopping queued phrase: {}", phrase.0);

        cmds.entity(id).despawn();
    }
}

fn stop_playing(
    // buttons: Single<&Gamepad>,
    mut playing_state: ResMut<NextState<PlayingState>>,
    playing: Query<&PlayingPhrase>,
) {
    // let start_button = GamepadButton::Start;
    //
    // if buttons.just_released(start_button) {
    //     playing_state.set(PlayingState::NotPlaying);
    // }

    if playing.iter().len() == 0 {
        info!("stopping playback");
        playing_state.set(PlayingState::NotPlaying);
    }
}

fn toggle_syncing(buttons: Single<&Gamepad>, mut playing_sync: ResMut<PlayingSyncPulse>) {
    let select_button = GamepadButton::Select;

    if buttons.just_released(select_button) && !buttons.pressed(GamepadButton::Mode) {
        playing_sync.0 = !playing_sync.0;
        info!("playing sync: {}", playing_sync.0);
    }
}
