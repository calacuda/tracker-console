use super::{Hoverable, Nudgable, Param, PulseDir, PulseShape, TimeParam, Volume};
use crate::{
    stepper::{step::Step, Channel, Cursor, InstrumentParams},
    InstrumentIndex,
};
use anyhow::{bail, Result};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

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

    fn set(&mut self, _to: f32) -> Result<()> {
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
            Some(Box::new(self.sweep)),
            Some(Box::new(self.sweep_time)),
            Some(Box::new(self.sweep_dir)),
            None,
        ]
    }

    fn nudge_param(&mut self, param_i: usize, up: bool) -> Result<()> {
        if param_i == 0 {
            self.shape.nudge(up)?
        } else if param_i == 1 {
            self.volume.nudge(up)?
        } else if param_i == 2 {
            self.time.nudge(up)?
        } else if param_i == 3 {
            self.dir.nudge(up)?
        } else if param_i == 4 {
            self.sweep.nudge(up)?
        } else if param_i == 5 {
            self.sweep_time.nudge(up)?
        } else if param_i == 6 {
            self.sweep_dir.nudge(up)?
        } else {
            bail!("invalid parameter selection.")
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PulseSweepChannel {
    steps: [Step<PulseSweepChannelParams>; 16],
    // dev_null: Step<PulseSweepChannelParams>,
}

impl Index<Cursor> for PulseSweepChannel {
    type Output = Step<PulseSweepChannelParams>;

    fn index(&self, index: Cursor) -> &Self::Output {
        // if let SoundChannelI::StepNum(i) = index.step_num {
        //     &self.steps[i]
        // } else {
        //     &self.dev_null
        // }
        &self.steps[index.step_num]
    }
}

impl IndexMut<Cursor> for PulseSweepChannel {
    fn index_mut(&mut self, index: Cursor) -> &mut Self::Output {
        // if let SoundChannelI::StepNum(i) = index.step_num {
        //     &mut self.steps[i]
        // } else {
        //     &mut self.dev_null
        // }
        &mut self.steps[index.step_num]
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

    fn nudge_step(&mut self, step_num: usize, up: bool, param_num: Option<usize>) -> Result<()> {
        self.steps[step_num].nudge(up, param_num)
    }
}
