use std::ops::{Index, IndexMut};

use crate::MidiNote;
use anyhow::{bail, Result};

use super::{step::Step, Cursor};

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
pub trait Channel<T>:
    Index<Cursor, Output = Step<T>>
    + IndexMut<Cursor>
    + Index<usize, Output = Step<T>>
    + IndexMut<usize>
where
    T: InstrumentParams,
{
    // fn get_steps(&self) -> [Option<Step<impl InstrumentParams>>; 16];
    fn get_steps(&self) -> [Step<T>; 16];
    fn nudge(&mut self, up: bool, param_num: Option<usize>) -> Result<()>;
    // fn nudge_down(&mut self, cursor: Cursor) -> Result<()>;
    // fn render_icon(&self, bevy_cmds: Commands, marker: impl Component);
    fn nudge_step(&mut self, step_num: usize, up: bool, param_num: Option<usize>) -> Result<()>;
}

pub trait Hoverable {
    /// the help menu text on the top of the screen
    fn tooltip(&self) -> String;
}
