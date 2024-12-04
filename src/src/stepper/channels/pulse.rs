use std::ops::{Index, IndexMut};

use super::{Nudgable, Param, PulseDir, PulseShape, TimeParam, Volume};
use crate::{
    stepper::{step::Step, Channel, Cursor, InstrumentParams},
    InstrumentIndex,
};
use anyhow::{bail, Result};

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseChannelParams {
    shape: PulseShape,
    volume: Volume,
    time: TimeParam,
    dir: PulseDir,
}

impl InstrumentParams for PulseChannelParams {
    type Instrument = InstrumentIndex;

    fn config_instrument(&self, _instrument: Self::Instrument) {
        todo!("write setting of instrument for PulseSweepChannelParams");
        // TODO: write setting of instrument for PulseSweepChannelParams
    }

    fn get_params(&self) -> [Option<Box<dyn Param>>; 8] {
        [
            Some(Box::new(self.shape)),
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
            self.shape.nudge(up)
        } else if param_i == 1 {
            self.volume.nudge(up)
        } else if param_i == 2 {
            self.time.nudge(up)
        } else if param_i == 3 {
            self.dir.nudge(up)
        // } else if param_i == 4 {
        //     self.sweep.nudge(up)
        // } else if param_i == 5 {
        //     self.sweep_time.nudge(up)
        // } else if param_i == 6 {
        //     self.sweep_dir.nudge(up)
        } else {
            bail!("invalid parameter selection.")
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseChannel {
    steps: [Step<PulseChannelParams>; 16],
    dev_null: Step<PulseChannelParams>,
}

impl Index<Cursor> for PulseChannel {
    type Output = Step<PulseChannelParams>;

    fn index(&self, index: Cursor) -> &Self::Output {
        // if let SoundChannelI::StepNum(i) = index.step_num {
        //     &self.steps[i]
        // } else {
        //     &self.dev_null
        // }
        &self.steps[index.step_num]
    }
}

impl IndexMut<Cursor> for PulseChannel {
    fn index_mut(&mut self, index: Cursor) -> &mut Self::Output {
        // if let SoundChannelI::StepNum(i) = index.step_num {
        //     &mut self.steps[i]
        // } else {
        //     &mut self.dev_null
        // }
        &mut self.steps[index.step_num]
    }
}

impl Index<usize> for PulseChannel {
    type Output = Step<PulseChannelParams>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.steps[index]
    }
}

impl IndexMut<usize> for PulseChannel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.steps[index]
    }
}

impl Channel<PulseChannelParams> for PulseChannel {
    fn nudge(&mut self, up: bool, param_num: Option<usize>) -> Result<()> {
        for ref mut step in self.steps {
            step.nudge(up, param_num)?
        }

        Ok(())
    }

    fn get_steps(&self) -> [Step<PulseChannelParams>; 16] {
        self.steps
    }

    fn nudge_step(&mut self, step_num: usize, up: bool, param_num: Option<usize>) -> Result<()> {
        Ok(())
    }
}
