use crate::MidiNote;
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
    /// handles rendering the icons for the params to the screen
    fn draw_icons(&self, bevy_cmds: Commands, marker: impl Component);
}

pub trait Channel {
    fn get_steps(&self) -> [Option<Step<impl InstrumentParams>>; 16];
    fn render_icon(&self, bevy_cmds: Commands, marker: impl Component);
    fn tooltip(&self) -> String;
}

#[derive(Clone, Default, Debug, Component)]
pub struct Pattern {
    pub c1: PulseSweepChanel,
    pub c2: PulseChanel,
    pub c3: WaveChanel,
    pub c4: NoiseChanel,
}

#[derive(Clone, Default, Debug)]
pub struct PulseSweepChanel {}

#[derive(Clone, Default, Debug)]
pub struct PulseChanel {}

// impl Channel for PulseChannel {}

#[derive(Clone, Default, Debug)]
pub struct WaveChanel {}

// impl Channel for WaveChannel {}

#[derive(Clone, Default, Debug)]
pub struct NoiseChanel {}

// impl Channel for NoiseChannel {}
