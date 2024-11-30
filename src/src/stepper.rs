use crate::{
    controls::MyGamepad, AllPatterns, MidiNote, ScreenState, StepperChannel, StepperChannelParam,
    GRAPH_X, GRAPH_Y,
};
use anyhow::Result;
use bevy::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct Step<T>
where
    T: InstrumentParams,
{
    pub note: MidiNote,
    pub params: T,
    pub muted: bool,
}

pub trait Param {
    fn name(&self) -> String;
    fn nudge_up(&mut self) -> Result<()>;
    fn nudge_down(&mut self) -> Result<()>;
    fn set(&mut self, to: f32) -> Result<()>;
    /// the help menu text on the top of the screen
    fn tooltip(&self) -> String;
}

pub trait InstrumentParams: Default + Clone + std::fmt::Debug {
    type Instrument;

    fn config_instrument(&self, instrument: Self::Instrument);
    /// retrieves the parameters
    fn get_params(&self) -> [Option<impl Param>; 8];
    // /// handles rendering the icons for the params to the screen
    // fn draw_icons(&self, bevy_cmds: Commands, marker: impl Component);
}

pub trait Channel {
    fn get_steps(&self) -> [Option<Step<impl InstrumentParams>>; 16];
    fn render_icon(&self, bevy_cmds: Commands, marker: impl Component);
    fn tooltip(&self) -> String;
}

#[derive(Clone, Copy, Default, Debug, Component)]
pub struct Pattern {
    pub c1: PulseSweepChannel,
    pub c2: PulseChannel,
    pub c3: WaveChannel,
    pub c4: NoiseChannel,
    pub name: usize,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseSweepChannel {}

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseChannel {}

// impl Channel for PulseChannel {}

#[derive(Clone, Copy, Default, Debug)]
pub struct WaveChannel {}

// impl Channel for WaveChannel {}

#[derive(Clone, Copy, Default, Debug)]
pub struct NoiseChannel {}

// impl Channel for NoiseChannel {}

#[derive(Clone, Debug, Resource)]
pub struct Patterns(AllPatterns);

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorMovement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum SoundChannel {
    PulseWSweep,
    PulseWOSweep,
    Wave,
    Noise,
}

#[derive(Clone, Debug)]
pub enum SoundChannelI {
    StepNum(usize),
    /// indecates that the cursusor is hovering over the channel selection buttons
    SoundChannel,
}

#[derive(Clone, Debug, Resource)]
pub struct Cursor {
    channel: SoundChannel,
    step_num: SoundChannelI,
    param_num: usize,
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
            .add_systems(
                Update,
                channel_movement.run_if(in_state(StepperChannelParam::Channels)),
            )
            .add_systems(
                Update,
                param_movement.run_if(in_state(StepperChannelParam::Params)),
            );
    }
}

fn stepper_setup(mut commands: Commands) {
    commands.insert_resource(Cursor {
        channel: SoundChannel::PulseWSweep,
        step_num: SoundChannelI::StepNum(0),
        param_num: 0,
    });
}

fn stepper_teardown(mut commands: Commands) {
    commands.remove_resource::<Cursor>();
}

// TODO: setting of step midi note.
// TODO: setting of params for single step
// TODO: setting of params for whole channel

fn param_movement(mut reader: EventReader<CursorMovement>, mut cursor: ResMut<Cursor>) {
    for event in reader.read() {
        match *event {
            CursorMovement::Up => {
                cursor.param_num += 8;
                cursor.param_num %= 16;
            }
            CursorMovement::Down => {
                cursor.param_num += 8;
                cursor.param_num %= 16;
            }
            CursorMovement::Right => {
                cursor.param_num += 1;
                cursor.param_num %= 16;
            }
            CursorMovement::Left => {
                if cursor.param_num > 0 {
                    cursor.param_num -= 1;
                    cursor.param_num %= 16;
                } else {
                    cursor.param_num = 15;
                }
            }
        }
    }
}

fn channel_movement(mut reader: EventReader<CursorMovement>, mut cursor: ResMut<Cursor>) {
    for event in reader.read() {
        match *event {
            CursorMovement::Up => {
                if let SoundChannelI::StepNum(ref mut i) = cursor.step_num {
                    *i += 8;
                    *i %= 16;
                } else {
                    cursor.channel = match cursor.channel {
                        SoundChannel::PulseWSweep => SoundChannel::Noise,
                        SoundChannel::PulseWOSweep => SoundChannel::PulseWSweep,
                        SoundChannel::Wave => SoundChannel::PulseWOSweep,
                        SoundChannel::Noise => SoundChannel::Wave,
                    };
                }
            }
            CursorMovement::Down => {
                if let SoundChannelI::StepNum(ref mut i) = cursor.step_num {
                    *i += 8;
                    *i %= 16;
                } else {
                    cursor.channel = match cursor.channel {
                        SoundChannel::PulseWSweep => SoundChannel::PulseWOSweep,
                        SoundChannel::PulseWOSweep => SoundChannel::Wave,
                        SoundChannel::Wave => SoundChannel::Noise,
                        SoundChannel::Noise => SoundChannel::PulseWSweep,
                    };
                }
            }
            CursorMovement::Right => {
                if let SoundChannelI::StepNum(ref mut i) = cursor.step_num {
                    if *i < 15 {
                        *i += 1;
                        *i %= 16;
                    } else {
                        cursor.step_num = SoundChannelI::SoundChannel;
                    }
                } else {
                    cursor.step_num = SoundChannelI::StepNum(15);
                }
            }
            CursorMovement::Left => {
                if let SoundChannelI::StepNum(ref mut i) = cursor.step_num {
                    if *i > 0 {
                        *i -= 1
                    } else {
                        cursor.step_num = SoundChannelI::SoundChannel;
                    }
                } else {
                    cursor.step_num = SoundChannelI::StepNum(0);
                }
            }
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
