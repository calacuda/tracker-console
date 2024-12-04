use crate::{
    controls::MyGamepad,
    graph::{Cursor as GraphCursor, Graph, TrackerNode, TrackerNodeData},
    AllPatterns, PatternIndex, ScreenState, StepperChannelParam, GRAPH_X, GRAPH_Y,
};
use bevy::prelude::*;
use channels::{
    noise::NoiseChannel, pulse::PulseChannel, pulse_sweep::PulseSweepChannel, wave::WaveChannel,
};
use std::{
    ops::{Index, IndexMut},
    time::{Duration, Instant},
};
use traits::*;

pub mod channels;
pub mod step;
pub mod traits;

#[derive(Clone, Copy, Default, Debug, Component)]
pub struct Pattern {
    pub c1: PulseSweepChannel,
    pub c2: PulseChannel,
    pub c3: WaveChannel,
    pub c4: NoiseChannel,
    pub name: usize,
}

#[derive(Clone, Debug, Resource)]
pub struct Patterns(AllPatterns);

impl Index<EditingPattern> for Patterns {
    type Output = Option<Pattern>;

    fn index(&self, index: EditingPattern) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<EditingPattern> for Patterns {
    // type Output = Option<Pattern>;
    fn index_mut(&mut self, index: EditingPattern) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorMovement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NudgeEvent {
    cursor: Cursor,
    up: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoundChannel {
    PulseWSweep,
    PulseWOSweep,
    Wave,
    Noise,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorLocation {
    // StepNum(usize),
    // /// indicates that the cursusor is hovering over the channel selection buttons
    // SoundChannel,
    SoundChannel,
    Params,
    Notes,
    // AStep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Resource)]
pub struct Cursor {
    channel: SoundChannel,
    location: CursorLocation,
    step_num: usize,
    param_num: usize,
}

#[derive(Clone, Debug, Resource)]
pub struct EditingPattern(PatternIndex);

#[derive(Clone, Debug, Resource)]
pub struct StepEditTimmer(Instant);

impl StepEditTimmer {
    fn reset(&mut self) {
        self.0 = Instant::now();
    }
}

pub struct StepperPlugin;

impl Plugin for StepperPlugin {
    fn build(&self, app: &mut App) {
        let pats: Patterns = Patterns([None; GRAPH_Y * GRAPH_X]);

        app.insert_resource(pats)
            .add_event::<CursorMovement>()
            .add_systems(OnEnter(ScreenState::Stepper), stepper_setup)
            .add_systems(OnExit(ScreenState::Stepper), stepper_teardown)
            .add_systems(
                Update,
                param_or_channel.run_if(in_state(ScreenState::Stepper)),
            )
            .add_systems(Update, movement.run_if(in_state(ScreenState::Stepper)))
            .add_systems(Update, nudge.run_if(in_state(ScreenState::Stepper)))
            .add_systems(Update, nudge_note.run_if(in_state(ScreenState::Stepper)))
            .add_systems(
                Update,
                handle_movement.run_if(in_state(ScreenState::Stepper)),
            );
    }
}

fn stepper_setup(
    mut commands: Commands,
    cursor: Res<GraphCursor>,
    graph: Res<Graph>,
    mut next_state: ResMut<NextState<ScreenState>>,
) {
    commands.insert_resource(Cursor {
        channel: SoundChannel::PulseWSweep,
        location: CursorLocation::Notes,
        step_num: 0,
        param_num: 0,
    });

    // add index of pattern being edited
    let Some(TrackerNode {
        node_dat: TrackerNodeData::Pattern(pattern_i),
        ..
    }) = graph[cursor.clone()]
    else {
        error!(
            "the cursor was not focused on a pattern node. unknown what pattern you want to edit"
        );
        next_state.set(ScreenState::Graph);
        return;
    };

    commands.insert_resource(EditingPattern(pattern_i));
    commands.insert_resource(StepEditTimmer(Instant::now()))
}

fn stepper_teardown(mut commands: Commands) {
    commands.remove_resource::<Cursor>();
    commands.remove_resource::<EditingPattern>();
    commands.remove_resource::<StepEditTimmer>();
}

fn nudge_note(
    pattern_i: Res<EditingPattern>,
    mut pats: ResMut<Patterns>,
    state: Res<State<StepperChannelParam>>,
    mut reader: EventReader<NudgeEvent>,
) {
    let Some(ref mut pattern) = pats[pattern_i.clone()] else {
        return;
    };

    for ev in reader.read() {
        let param = match **state {
            StepperChannelParam::Channels => None,
            StepperChannelParam::Params => Some(ev.cursor.param_num),
        };

        if let Err(e) = match ev.cursor.location {
            CursorLocation::Params | CursorLocation::Notes => {
                match ev.cursor.channel {
                    SoundChannel::PulseWSweep => pattern.c1[ev.cursor.step_num].nudge(ev.up, param),
                    SoundChannel::PulseWOSweep => {
                        pattern.c2[ev.cursor.step_num].nudge(ev.up, param)
                    }
                    // SoundChannel::Wave => pattern.c3[ev.cursor.step_num].nudge(ev.up, param),
                    // SoundChannel::Noise => pattern.c4[ev.cursor.step_num].nudge(ev.up, param),
                    _ => todo!("write nudge for a single step, for {:?}", ev.cursor.channel),
                }
            }
            CursorLocation::SoundChannel => match ev.cursor.channel {
                SoundChannel::PulseWSweep => pattern.c1.nudge(ev.up, param),
                SoundChannel::PulseWOSweep => pattern.c2.nudge(ev.up, param),
                // SoundChannel::Wave => pattern.c3[ev.cursor.step_num].nudge(ev.up, param),
                // SoundChannel::Noise => pattern.c4[ev.cursor.step_num].nudge(ev.up, param),
                _ => todo!(
                    "write nudge for a whole channel for {:?}",
                    ev.cursor.channel
                ),
            },
        } {
            error!("{e}");
        }
    }
}

// TODO: setting of step midi note.
fn nudge(
    cursor: Res<Cursor>,
    // graph: Res<Graph>,
    // pattern_i: Res<EditingPattern>,
    mut edit_timmer: ResMut<StepEditTimmer>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut emitter: EventWriter<NudgeEvent>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let l = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::LeftTrigger,
    };

    let r = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::RightTrigger,
    };

    // if both left and right pressed restart timmer.
    if buttons.pressed(l) && buttons.pressed(r) {
        edit_timmer.reset();
    }

    let cooldown = Duration::from_secs_f32(0.125);

    // if left shoulder button pressed decrement
    if buttons.pressed(l) && edit_timmer.0.elapsed() > cooldown {
        // edit_timmer.reset();
        // if let Ok(ref mut step) pattern[(cursor)]
        emitter.send(NudgeEvent {
            cursor: cursor.clone(),
            up: false,
        });
    }
    // if right shoulder button pressed increment
    if buttons.pressed(r) && edit_timmer.0.elapsed() > cooldown {
        // edit_timmer.reset();
        // if let Ok(ref mut step) pattern[(cursor)]
        emitter.send(NudgeEvent {
            cursor: cursor.clone(),
            up: true,
        });
    }
}

fn handle_movement(mut reader: EventReader<CursorMovement>, mut cursor: ResMut<Cursor>) {
    for event in reader.read() {
        match *event {
            CursorMovement::Up => match cursor.location {
                CursorLocation::SoundChannel => {
                    cursor.channel = match cursor.channel {
                        SoundChannel::PulseWSweep => SoundChannel::Noise,
                        SoundChannel::PulseWOSweep => SoundChannel::PulseWSweep,
                        SoundChannel::Wave => SoundChannel::PulseWOSweep,
                        SoundChannel::Noise => SoundChannel::Wave,
                    }
                }
                CursorLocation::Params => {
                    cursor.param_num += 8;
                    cursor.param_num %= 16;
                }
                CursorLocation::Notes => {
                    cursor.step_num += 8;
                    cursor.step_num %= 16;
                }
            },
            CursorMovement::Down => match cursor.location {
                CursorLocation::SoundChannel => {
                    cursor.channel = match cursor.channel {
                        SoundChannel::PulseWSweep => SoundChannel::PulseWOSweep,
                        SoundChannel::PulseWOSweep => SoundChannel::Wave,
                        SoundChannel::Wave => SoundChannel::Noise,
                        SoundChannel::Noise => SoundChannel::PulseWSweep,
                    };
                }
                CursorLocation::Params => {
                    cursor.param_num += 8;
                    cursor.param_num %= 16;
                }
                CursorLocation::Notes => {
                    cursor.step_num += 8;
                    cursor.step_num %= 16;
                }
            },
            CursorMovement::Right => match cursor.location {
                CursorLocation::SoundChannel => {
                    cursor.location = CursorLocation::Notes;
                    cursor.step_num = 0;
                }
                CursorLocation::Params => {
                    cursor.param_num += 1;
                    cursor.param_num %= 16;
                }
                CursorLocation::Notes => {
                    if cursor.step_num < 15 {
                        cursor.step_num += 1;
                        cursor.step_num %= 16;
                    } else {
                        cursor.location = CursorLocation::SoundChannel;
                    }
                }
            },
            CursorMovement::Left => match cursor.location {
                CursorLocation::SoundChannel => {
                    cursor.location = CursorLocation::Notes;
                    cursor.step_num = 15;
                }
                CursorLocation::Params => {
                    if cursor.param_num > 0 {
                        cursor.param_num -= 1;
                    } else {
                        cursor.param_num = 15;
                    }
                }
                CursorLocation::Notes => {
                    if cursor.step_num > 0 {
                        cursor.step_num -= 1;
                    } else {
                        cursor.location = CursorLocation::SoundChannel;
                    }
                }
            },
        }
    }
}

fn movement(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut emitter: EventWriter<CursorMovement>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let up = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadUp,
    };
    let down = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadDown,
    };
    let left = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };

    let pressed = [
        buttons.just_pressed(up),
        buttons.just_pressed(down),
        buttons.just_pressed(left),
        buttons.just_pressed(right),
    ];

    if pressed.iter().filter(|b| **b).count() != 1 {
        // more then one dir button pressed
        return;
    }

    let events = [
        CursorMovement::Up,
        CursorMovement::Down,
        CursorMovement::Left,
        CursorMovement::Right,
    ];

    if let Some(i) = pressed.iter().position(|b| *b) {
        emitter.send(events[i]);
    }
}

fn param_or_channel(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut next_state: ResMut<NextState<StepperChannelParam>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let a = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if buttons.just_pressed(a) {
        next_state.set(StepperChannelParam::Params);
    } else if buttons.just_released(a) {
        next_state.set(StepperChannelParam::Channels);
    }
}
