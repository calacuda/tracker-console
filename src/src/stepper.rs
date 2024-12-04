use crate::{
    controls::MyGamepad,
    graph::{Cursor as GraphCursor, Graph, TrackerNode, TrackerNodeData},
    AllPatterns, InstrumentIndex, MidiNote, PatternIndex, ScreenState, StepperChannel,
    StepperChannelParam, GRAPH_X, GRAPH_Y,
};
use anyhow::{bail, ensure, Result};
use bevy::prelude::*;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    time::{Duration, Instant},
};

pub trait Nudgable {
    fn nudge_up(&mut self) -> Result<()>;
    fn nudge_down(&mut self) -> Result<()>;
    fn nudge(&mut self, up: bool) -> Result<()> {
        if up {
            self.nudge_up()
        } else {
            self.nudge_down()
        }
    }
}

impl Nudgable for MidiNote {
    fn nudge_up(&mut self) -> Result<()> {
        if *self < 127 {
            *self += 1;
            Ok(())
        } else {
            bail!("Midi Note already as high as possible. not increasing.");
        }
    }

    fn nudge_down(&mut self) -> Result<()> {
        if *self > 0 {
            *self -= 1;
            Ok(())
        } else {
            bail!("Midi Note already as low as possible. not increasing.");
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Step<T>
where
    T: InstrumentParams,
{
    pub note: MidiNote,
    pub params: T,
    pub muted: bool,
}

impl<T> Step<T>
where
    T: InstrumentParams,
{
    fn nudge(&mut self, up: bool, param_num: Option<usize>) -> Result<()> {
        // match cursor.step_num {
        //     SoundChannelI::StepNum(_i) => {}
        //     SoundChannelI::
        // }

        if let Some(param_i) = param_num {
            if let Err(e) = self.params.nudge_param(param_i, up) {
                // param.nudge(up);
                // } else {
                bail!("the parameter numbered {param_i} is not a parameter of this step. got error, {e}.")
            }
        } else {
            self.note.nudge(up);
        }

        Ok(())
    }
}

pub trait Param: Hoverable + Nudgable {
    fn name(&self) -> String;

    fn set(&mut self, to: f32) -> Result<()>;
    // /// the help menu text on the top of the screen
    // fn tooltip(&self) -> String;
}

pub trait InstrumentParams: Default + Clone + std::fmt::Debug
// + Index<usize, Output = Option<Box<dyn Param>>> + IndexMut<usize>
{
    type Instrument;

    fn config_instrument(&self, instrument: Self::Instrument);
    /// retrieves the parameters
    fn get_params(&self) -> [Option<Box<dyn Param>>; 8];
    // /// handles rendering the icons for the params to the screen
    // fn draw_icons(&self, bevy_cmds: Commands, marker: impl Component);
    fn nudge_param(&mut self, param_i: usize, up: bool) -> Result<()>;
}

// : Index<Cursor, Output = Step<T>> + IndexMut<Cursor>
pub trait Channel<T>
where
    T: InstrumentParams,
{
    // fn get_steps(&self) -> [Option<Step<impl InstrumentParams>>; 16];
    fn get_steps(&self) -> [Step<T>; 16];
    fn nudge(&mut self, up: bool, cursor: Option<usize>) -> Result<()>;
    // fn nudge_down(&mut self, cursor: Cursor) -> Result<()>;
    // fn render_icon(&self, bevy_cmds: Commands, marker: impl Component);
}

pub trait Hoverable {
    /// the help menu text on the top of the screen
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

impl Index<(SoundChannel, Cursor)> for Pattern {
    type Output = Step<impl InstrumentParams>;

    fn index(&self, index: (SoundChannel, Cursor)) -> &Self::Output {
        match index.0 {
            SoundChannel::PulseWSweep => &self.c1[index.1],
            _ => todo!("write the rest"),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum PulseShape {
    Eighth,
    Quarter,
    #[default]
    Half,
    ThreeQuarters,
}

impl Display for PulseShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eighth => write!(f, "12%"),
            Self::Quarter => write!(f, "25%"),
            Self::Half => write!(f, "50%"),
            Self::ThreeQuarters => write!(f, "75%"),
        }
    }
}

impl Hoverable for PulseShape {
    fn tooltip(&self) -> String {
        format!("{}: {}", self.name(), self)
    }
}

impl Param for PulseShape {
    fn name(&self) -> String {
        "SHAPE".into()
    }
    fn set(&mut self, to: f32) -> Result<()> {
        ensure!(
            to > 0.0 && to < 1.0,
            "cannot set volume to {to}. as it is out of range. (must be between 0.0 & 1.0)"
        );

        *self = if to <= 1. / 8. {
            Self::Eighth
        } else if to <= 1. / 4. {
            Self::Quarter
        } else if to <= 1. / 2. {
            Self::Half
        } else if to <= 3. / 4. {
            Self::ThreeQuarters
        } else {
            bail!("somthing unknown went wrong in the set method of PulseShape")
        };

        Ok(())
    }
}

impl Nudgable for PulseShape {
    fn nudge_up(&mut self) -> Result<()> {
        *self = match *self {
            Self::ThreeQuarters => bail!("Shape is MAX. cannot increase."),
            Self::Half => Self::ThreeQuarters,
            Self::Quarter => Self::Half,
            Self::Eighth => Self::Quarter,
        };

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        *self = match *self {
            Self::Eighth => bail!("Shape is MIN. cannot Decrease."),
            Self::Quarter => Self::Eighth,
            Self::Half => Self::Quarter,
            Self::ThreeQuarters => Self::Half,
        };

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Volume {
    value: u8,
}

impl Hoverable for Volume {
    fn tooltip(&self) -> String {
        format!("{}: {}%", self.name(), self.value)
    }
}

impl Param for Volume {
    fn name(&self) -> String {
        "VOL".into()
    }

    fn set(&mut self, to: f32) -> Result<()> {
        ensure!(
            to > 0.0 && to < 100.0,
            "cannot set volume to {to}. as it is out of range. (must be between 0.0 & 100.0)"
        );

        if to < 99.5 {
            self.value = to.floor() as u8;
        } else {
            self.value = 100;
        }

        Ok(())
    }
}

impl Nudgable for Volume {
    fn nudge_up(&mut self) -> Result<()> {
        if self.value <= 95 {
            self.value += 5;
            Ok(())
        } else {
            self.value = 100;
            bail!("volume already at MAX. cannot increase.");
        }
    }

    fn nudge_down(&mut self) -> Result<()> {
        if self.value >= 5 {
            self.value -= 5;
            Ok(())
        } else {
            bail!("volume already at MIN. cannot decrease.");
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum TimeParam {
    OneFiveMS,
    ThreeOneMS,
    FourSixMS,
    #[default]
    SixTwoMS,
    SevenEightMS,
    NineThreeMS,
    OneS,
    Off,
}

impl Display for TimeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::OneFiveMS => write!(f, "150-MS"),
            Self::ThreeOneMS => write!(f, "310-MS"),
            Self::FourSixMS => write!(f, "460-MS"),
            Self::SixTwoMS => write!(f, "620-MS"),
            Self::SevenEightMS => write!(f, "780-MS"),
            Self::NineThreeMS => write!(f, "930-MS"),
            Self::OneS => write!(f, "1-S"),
            Self::Off => write!(f, "OFF"),
        }
    }
}

impl Hoverable for TimeParam {
    fn tooltip(&self) -> String {
        format!("{}: {}", self.name(), self)
    }
}

impl Param for TimeParam {
    fn name(&self) -> String {
        "TIME".into()
    }

    fn set(&mut self, to: f32) -> Result<()> {
        ensure!(
            to > 0.0 && to < 100.0,
            "cannot set time to {to}. as it is out of range. (must be between 0.0 & 100.0)"
        );

        let to = to.round();
        let fraction = 100. / 8.;

        *self = if to <= 1. * fraction {
            Self::OneFiveMS
        } else if to <= 2. * fraction {
            Self::ThreeOneMS
        } else if to <= 3. * fraction {
            Self::FourSixMS
        } else if to <= 4. * fraction {
            Self::SixTwoMS
        } else if to <= 5. * fraction {
            Self::SevenEightMS
        } else if to <= 6. * fraction {
            Self::NineThreeMS
        } else if to <= 7. * fraction {
            Self::OneS
        } else if to <= 100.0 {
            Self::Off
        } else {
            bail!("Unknown error ocured in  TimeParam's set method.");
        };

        Ok(())
    }
}

impl Nudgable for TimeParam {
    fn nudge_up(&mut self) -> Result<()> {
        *self = match *self {
            Self::OneFiveMS => Self::ThreeOneMS,
            Self::ThreeOneMS => Self::FourSixMS,
            Self::FourSixMS => Self::SixTwoMS,
            Self::SixTwoMS => Self::SevenEightMS,
            Self::SevenEightMS => Self::NineThreeMS,
            Self::NineThreeMS => Self::OneS,
            Self::OneS => Self::Off,
            Self::Off => bail!("Time already at MAX value. Cannot increase further."),
        };

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        *self = match *self {
            Self::OneFiveMS => bail!("Time already at MIN value. Cannot decrease further."),
            Self::ThreeOneMS => Self::OneFiveMS,
            Self::FourSixMS => Self::ThreeOneMS,
            Self::SixTwoMS => Self::FourSixMS,
            Self::SevenEightMS => Self::SixTwoMS,
            Self::NineThreeMS => Self::SevenEightMS,
            Self::OneS => Self::NineThreeMS,
            Self::Off => Self::OneS,
        };

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseDir {
    up: bool,
}

impl Display for PulseDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.up {
            write!(f, "UP")
        } else {
            write!(f, "DOWN")
        }
    }
}

impl Hoverable for PulseDir {
    fn tooltip(&self) -> String {
        format!("DIRECTION: {self}")
    }
}

impl Param for PulseDir {
    fn name(&self) -> String {
        "DIR".into()
    }

    fn set(&mut self, to: f32) -> Result<()> {
        ensure!(
            to > 0.0 && to < 100.0,
            "cannot set direction to {to}. as it is out of range. (must be between 0.0 & 100.0)"
        );

        self.up = to < 50.;

        Ok(())
    }
}

impl Nudgable for PulseDir {
    fn nudge_up(&mut self) -> Result<()> {
        self.up = !self.up;

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        self.nudge_up()
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Sweep {
    value: u8,
}

impl Display for Sweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value == 1 {
            write!(f, "1")
        } else if self.value == 2 {
            write!(f, "2")
        } else if self.value == 4 {
            write!(f, "3")
        } else if self.value == 8 {
            write!(f, "4")
        } else if self.value == 16 {
            write!(f, "5")
        } else if self.value == 32 {
            write!(f, "6")
        } else if self.value == 64 {
            write!(f, "7")
        } else {
            write!(f, "0")
        }
    }
}

impl Hoverable for Sweep {
    fn tooltip(&self) -> String {
        format!("SWEEP BITS: {self}")
    }
}

impl Param for Sweep {
    fn name(&self) -> String {
        "SWEEP".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("Write this function... or dont");
        // Ok(())
    }
}

impl Nudgable for Sweep {
    fn nudge_up(&mut self) -> Result<()> {
        if self.value < 64 && self.value > 0 {
            self.value *= 2;
        } else if self.value == 0 {
            self.value = 1
        } else {
            bail!("sweep was too high");
        }

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        if self.value > 0 {
            self.value /= 2;
        } else if self.value == 1 {
            self.value = 0;
        } else {
            bail!("sweep was too high");
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum SweepTime {
    #[default]
    Off,
    SevenMS,
    FifteenMS,
    TwentyThreeMS,
    ThirtyOneMS,
    ThirtyNineMS,
    FortySixMS,
    FiftyFourMS,
}

impl Display for SweepTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Off => write!(f, "OFF"),
            Self::SevenMS => write!(f, "7 MS"),
            Self::FifteenMS => write!(f, "15 MS"),
            Self::TwentyThreeMS => write!(f, "23 MS"),
            Self::ThirtyOneMS => write!(f, "31 MS"),
            Self::ThirtyNineMS => write!(f, "39 MS"),
            Self::FortySixMS => write!(f, "46 MS"),
            Self::FiftyFourMS => write!(f, "54 MS"),
        }
    }
}

impl Hoverable for SweepTime {
    fn tooltip(&self) -> String {
        format!("SWEEP TIME: {self}")
    }
}

impl Param for SweepTime {
    fn name(&self) -> String {
        "TIME".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("write this")
    }
}

impl Nudgable for SweepTime {
    fn nudge_up(&mut self) -> Result<()> {
        *self = match self {
            Self::Off => Self::SevenMS,
            Self::SevenMS => Self::FifteenMS,
            Self::FifteenMS => Self::TwentyThreeMS,
            Self::TwentyThreeMS => Self::ThirtyOneMS,
            Self::ThirtyOneMS => Self::ThirtyNineMS,
            Self::ThirtyNineMS => Self::FortySixMS,
            Self::FortySixMS => Self::FiftyFourMS,
            Self::FiftyFourMS => bail!("Sweep Time is MAXED. not increasing."),
        };

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        *self = match self {
            Self::Off => bail!("Sweep Time is MIN-ED. not increasing."),
            Self::SevenMS => Self::Off,
            Self::FifteenMS => Self::SevenMS,
            Self::TwentyThreeMS => Self::FifteenMS,
            Self::ThirtyOneMS => Self::TwentyThreeMS,
            Self::ThirtyNineMS => Self::ThirtyOneMS,
            Self::FortySixMS => Self::ThirtyNineMS,
            Self::FiftyFourMS => Self::FortySixMS,
        };

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct SweepDir {
    up: bool,
}

impl Display for SweepDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.up {
            write!(f, "UP")
        } else {
            write!(f, "DOWN")
        }
    }
}

impl Hoverable for SweepDir {
    fn tooltip(&self) -> String {
        format!("SWEEP DIRECTION: {self}")
    }
}

impl Param for SweepDir {
    fn name(&self) -> String {
        "DIR".into()
    }

    fn set(&mut self, to: f32) -> Result<()> {
        todo!("write this...")
    }
}

impl Nudgable for SweepDir {
    fn nudge_up(&mut self) -> Result<()> {
        self.up = !self.up;

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        self.up = !self.up;

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseSweepChannelParams {
    shape: PulseShape,
    volume: Volume,
    time: TimeParam,
    dir: PulseDir,
    sweep: Sweep,
    sweep_time: SweepTime,
    sweep_dir: SweepDir,
}

impl InstrumentParams for PulseSweepChannelParams {
    type Instrument = InstrumentIndex;

    fn config_instrument(&self, instrument: Self::Instrument) {
        todo!("write setting of instrument for PulseSweepChannelParams");
        // TODO: write setting of instrument for PulseSweepChannelParams
    }

    fn get_params(&self) -> [Option<Box<dyn Param>>; 8] {
        [
            Some(Box::new(self.shape)),
            Some(Box::new(self.volume)),
            Some(Box::new(self.time)),
            Some(Box::new(self.dir)),
            Some(Box::new(self.sweep)),
            Some(Box::new(self.sweep_time)),
            Some(Box::new(self.sweep_dir)),
            None,
        ]
    }

    fn nudge_param(&mut self, param_i: usize, up: bool) -> Result<()> {
        if param_i == 0 {
            self.shape.nudge(up)
        } else if param_i == 1 {
            self.volume.nudge(up)
        } else if param_i == 2 {
            self.time.nudge(up)
        } else if param_i == 3 {
            self.dir.nudge(up)
        } else if param_i == 4 {
            self.sweep.nudge(up)
        } else if param_i == 5 {
            self.sweep_time.nudge(up)
        } else if param_i == 6 {
            self.sweep_dir.nudge(up)
        } else {
            bail!("invalid parameter selection.")
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseSweepChannel {
    steps: [Step<PulseSweepChannelParams>; 16],
    dev_null: Step<PulseSweepChannelParams>,
}

impl Index<Cursor> for PulseSweepChannel {
    type Output = Step<PulseSweepChannelParams>;

    fn index(&self, index: Cursor) -> &Self::Output {
        if let SoundChannelI::StepNum(i) = index.step_num {
            &self.steps[i]
        } else {
            &self.dev_null
        }
    }
}

impl IndexMut<Cursor> for PulseSweepChannel {
    fn index_mut(&mut self, index: Cursor) -> &mut Self::Output {
        if let SoundChannelI::StepNum(i) = index.step_num {
            &mut self.steps[i]
        } else {
            &mut self.dev_null
        }
    }
}

impl Index<usize> for PulseSweepChannel {
    type Output = Step<PulseSweepChannelParams>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.steps[index]
    }
}

impl IndexMut<usize> for PulseSweepChannel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.steps[index]
    }
}

impl Channel<PulseSweepChannelParams> for PulseSweepChannel {
    fn get_steps(&self) -> [Step<PulseSweepChannelParams>; 16] {
        self.steps
    }

    fn nudge(&mut self, up: bool, param_num: Option<usize>) -> Result<()> {
        // if let SoundChannelI::StepNum(i) = cursor.step_num {
        for ref mut step in self.steps {
            step.nudge(up, param_num)?
        }

        Ok(())
        // } else {}
    }
    // fn render_icon(&self, bevy_cmds: Commands, marker: impl Component) {
    //     // make PulseSweepChannel able to render its icon
    // }

    // fn tooltip(&self) -> String {}
}

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
    /// indicates that the cursusor is hovering over the channel selection buttons
    SoundChannel,
}

#[derive(Clone, Debug, Resource)]
pub struct Cursor {
    channel: SoundChannel,
    step_num: SoundChannelI,
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

fn stepper_setup(
    mut commands: Commands,
    cursor: Res<GraphCursor>,
    graph: Res<Graph>,
    mut next_state: ResMut<NextState<ScreenState>>,
) {
    commands.insert_resource(Cursor {
        channel: SoundChannel::PulseWSweep,
        step_num: SoundChannelI::StepNum(0),
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

// TODO: setting of step midi note.
fn set_midi_note(
    cursor: Res<Cursor>,
    graph: Res<Graph>,
    mut pats: ResMut<Patterns>,
    state: Res<State<StepperChannel>>,
    pattern_i: Res<EditingPattern>,
    mut edit_timmer: ResMut<StepEditTimmer>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let SoundChannelI::StepNum(i) = cursor.step_num else {
        return;
    };

    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let Some(ref mut pattern) = pats[pattern_i.clone()] else {
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

    if buttons.pressed(l) && edit_timmer.0.elapsed() > cooldown {
        // edit_timmer.reset();
        // if let Ok(ref mut step) pattern[(cursor)]
        // TODO: emit a midi note nudge event
    }
    // if left shoulder button pressed decrement
    // if right shoulder button pressed increment
    // pattern[cursor]
}

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
