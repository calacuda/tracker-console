use super::{Hoverable, Nudgable, Param, PulseDir, TimeParam, Volume};
use crate::{
    stepper::{step::Step, Channel, Cursor, InstrumentParams},
    InstrumentIndex,
};
use anyhow::{bail, ensure, Result};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub struct NoiseMode {
    a: bool,
}

impl Default for NoiseMode {
    fn default() -> Self {
        NoiseMode { a: true }
    }
}

impl Hoverable for NoiseMode {
    fn tooltip(&self) -> String {
        format!("{}: {}", self.name(), if self.a { "A" } else { "B" })
    }
}

impl Nudgable for NoiseMode {
    fn nudge_up(&mut self) -> Result<()> {
        ensure!(!self.a, "MODE is MAX");
        self.a = false;
        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        ensure!(self.a, "MODE is MIN");
        self.a = true;
        Ok(())
    }
}

impl Param for NoiseMode {
    fn name(&self) -> String {
        "MODE".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("this does not need to be written")
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NoiseChannelParams {
    mode: NoiseMode,
    volume: Volume,
    time: TimeParam,
    dir: PulseDir,
}

impl InstrumentParams for NoiseChannelParams {
    type Instrument = InstrumentIndex;

    fn config_instrument(&self, _instrument: Self::Instrument) {
        todo!("configure the instrument for noise channel");
    }

    fn get_params(&self) -> [Option<Box<dyn Param>>; 8] {
        [
            Some(Box::new(self.mode)),
            Some(Box::new(self.volume)),
            Some(Box::new(self.time)),
            Some(Box::new(self.dir)),
            None,
            None,
            None,
            None,
        ]
    }

    fn nudge_param(&mut self, param_i: usize, up: bool) -> Result<()> {
        if param_i == 0 {
            self.mode.nudge(up)?
        } else if param_i == 1 {
            self.volume.nudge(up)?
        } else if param_i == 2 {
            self.time.nudge(up)?
        } else if param_i == 3 {
            self.dir.nudge(up)?
        } else {
            bail!("invalid parameter selection.")
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NoiseChannel {
    steps: [Step<NoiseChannelParams>; 16],
}

impl Index<Cursor> for NoiseChannel {
    type Output = Step<NoiseChannelParams>;

    fn index(&self, index: Cursor) -> &Self::Output {
        &self.steps[index.step_num]
    }
}

impl IndexMut<Cursor> for NoiseChannel {
    fn index_mut(&mut self, index: Cursor) -> &mut Self::Output {
        &mut self.steps[index.step_num]
    }
}

impl Index<usize> for NoiseChannel {
    type Output = Step<NoiseChannelParams>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.steps[index]
    }
}

impl IndexMut<usize> for NoiseChannel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.steps[index]
    }
}

impl Channel<NoiseChannelParams> for NoiseChannel {
    fn get_steps(&self) -> [Step<NoiseChannelParams>; 16] {
        self.steps
    }

    fn nudge(&mut self, up: bool, param_num: Option<usize>) -> Result<()> {
        for ref mut step in self.steps {
            step.nudge(up, param_num)?
        }

        Ok(())
    }

    fn nudge_step(&mut self, step_num: usize, up: bool, param_num: Option<usize>) -> Result<()> {
        self.steps[step_num].nudge(up, param_num)
    }
}
