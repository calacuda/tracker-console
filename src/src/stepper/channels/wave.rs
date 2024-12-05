use super::{Hoverable, Nudgable, Param};
use crate::{
    stepper::{step::Step, Channel, Cursor, InstrumentParams},
    InstrumentIndex,
};
use anyhow::{bail, Ok, Result};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Default, Debug)]
pub enum WaveShapeType {
    #[default]
    Sin,
    Saw,
    Sqr,
}

impl Display for WaveShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sin => write!(f, "SIN"),
            Self::Saw => write!(f, "SAW"),
            Self::Sqr => write!(f, "SQUARE"),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct WaveShape {
    shape: WaveShapeType,
    voice: bool,
}

impl Nudgable for WaveShape {
    fn nudge_up(&mut self) -> Result<()> {
        self.shape = match self.shape {
            WaveShapeType::Sin => WaveShapeType::Saw,
            WaveShapeType::Saw => WaveShapeType::Sqr,
            WaveShapeType::Sqr => bail!("WaveShape is at its MAX."),
        };

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        self.shape = match self.shape {
            WaveShapeType::Sin => bail!("WaveShape is at its MIN."),
            WaveShapeType::Saw => WaveShapeType::Sin,
            WaveShapeType::Sqr => WaveShapeType::Saw,
        };

        Ok(())
    }
}

impl Hoverable for WaveShape {
    fn tooltip(&self) -> String {
        format!(
            "{} {}: {}",
            self.name(),
            if self.voice { "A" } else { "B" },
            self.shape
        )
    }
}

impl Param for WaveShape {
    fn name(&self) -> String {
        "SHAPE".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("this does not need to be written")
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum WaveTypeType {
    #[default]
    A,
    B,
    C,
    D,
}

impl Display for WaveTypeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct WaveType {
    wave_type: WaveTypeType,
    voice: bool,
}

impl Nudgable for WaveType {
    fn nudge_up(&mut self) -> Result<()> {
        self.wave_type = match self.wave_type {
            WaveTypeType::A => WaveTypeType::B,
            WaveTypeType::B => WaveTypeType::C,
            WaveTypeType::C => WaveTypeType::D,
            WaveTypeType::D => bail!("WaveType is already at MAX."),
        };

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        self.wave_type = match self.wave_type {
            WaveTypeType::A => bail!("WaveType is already at MIN."),
            WaveTypeType::B => WaveTypeType::A,
            WaveTypeType::C => WaveTypeType::B,
            WaveTypeType::D => WaveTypeType::C,
        };

        Ok(())
    }
}

impl Hoverable for WaveType {
    fn tooltip(&self) -> String {
        format!(
            "{} {}: {}",
            self.name(),
            if self.voice { "A" } else { "B" },
            self.wave_type
        )
    }
}

impl Param for WaveType {
    fn name(&self) -> String {
        "TYPE".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("this does not need to be written")
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Voice {
    #[default]
    A,
    B,
    Both,
}

impl Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::Both => write!(f, "A + B"),
        }
    }
}

impl Nudgable for Voice {
    fn nudge_up(&mut self) -> Result<()> {
        *self = match *self {
            Self::A => Self::B,
            Self::B => Self::Both,
            Self::Both => bail!("Voice is maxed"),
        };

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        *self = match *self {
            Self::A => bail!("Voice is MIN"),
            Self::B => Self::A,
            Self::Both => Self::B,
        };

        Ok(())
    }
}

impl Hoverable for Voice {
    fn tooltip(&self) -> String {
        format!("{}: {self}", self.name())
    }
}

impl Param for Voice {
    fn name(&self) -> String {
        "VOICE".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("this does not need to be written")
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct WaveVolume {
    level: usize,
}

impl Nudgable for WaveVolume {
    fn nudge_up(&mut self) -> Result<()> {
        if self.level < 100 {
            self.level += 25
        } else {
            self.level = 100;
            bail!("wave channel volume is max")
        }

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        if self.level > 0 {
            self.level -= 25
        } else {
            self.level = 0;
            bail!("wave channel volume is min")
        }

        Ok(())
    }
}

impl Hoverable for WaveVolume {
    fn tooltip(&self) -> String {
        format!("VOLUME: {}%", self.level)
    }
}

impl Param for WaveVolume {
    fn name(&self) -> String {
        "VOL".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("this does not need to be written")
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct WaveAttack {
    atk: u8,
}

impl Nudgable for WaveAttack {
    fn nudge_up(&mut self) -> Result<()> {
        if self.atk < 16 {
            self.atk += 1;
        } else {
            bail!("wave table attack is MAX")
        }

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        if self.atk > 0 {
            self.atk -= 1;
        } else {
            bail!("wave table attack is MIN")
        }

        Ok(())
    }
}

impl Hoverable for WaveAttack {
    fn tooltip(&self) -> String {
        format!(
            "ATTACK: {}",
            if self.atk > 0 {
                format!("{}", self.atk)
            } else {
                "OFF".into()
            }
        )
    }
}

impl Param for WaveAttack {
    fn name(&self) -> String {
        "ATTACK".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("this does not need to be written")
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct WaveDecay {
    decay: u8,
}

impl Nudgable for WaveDecay {
    fn nudge_up(&mut self) -> Result<()> {
        if self.decay < 16 {
            self.decay += 1;
        } else {
            bail!("wave table attack is MAX")
        }

        Ok(())
    }

    fn nudge_down(&mut self) -> Result<()> {
        if self.decay > 0 {
            self.decay -= 1;
        } else {
            bail!("wave table attack is MIN")
        }

        Ok(())
    }
}

impl Hoverable for WaveDecay {
    fn tooltip(&self) -> String {
        format!(
            "DECAY: {}",
            if self.decay > 0 {
                format!("{}", 17 - self.decay)
            } else {
                "OFF".into()
            }
        )
    }
}

impl Param for WaveDecay {
    fn name(&self) -> String {
        "ATTACK".into()
    }

    fn set(&mut self, _to: f32) -> Result<()> {
        todo!("this does not need to be written")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WaveChannelParams {
    shape_a: WaveShape,
    type_a: WaveType,
    shape_b: WaveShape,
    type_b: WaveType,
    voice: Voice,
    volume: WaveVolume,
    attack: WaveAttack,
    decay: WaveDecay,
}

impl Default for WaveChannelParams {
    fn default() -> Self {
        Self {
            shape_a: WaveShape {
                shape: WaveShapeType::Sin,
                voice: true,
            },
            type_a: WaveType {
                wave_type: WaveTypeType::A,
                voice: true,
            },
            shape_b: WaveShape {
                shape: WaveShapeType::Saw,
                voice: false,
            },
            type_b: WaveType {
                wave_type: WaveTypeType::A,
                voice: false,
            },
            voice: Voice::default(),
            volume: WaveVolume::default(),
            attack: WaveAttack::default(),
            decay: WaveDecay::default(),
        }
    }
}

impl InstrumentParams for WaveChannelParams {
    type Instrument = InstrumentIndex;

    fn config_instrument(&self, _instrument: Self::Instrument) {
        todo!("write setting of instrument for WaveChannelParams");
    }

    fn get_params(&self) -> [Option<Box<dyn Param>>; 8] {
        [
            Some(Box::new(self.shape_a)),
            Some(Box::new(self.type_a)),
            Some(Box::new(self.shape_b)),
            Some(Box::new(self.type_b)),
            Some(Box::new(self.voice)),
            Some(Box::new(self.volume)),
            Some(Box::new(self.attack)),
            Some(Box::new(self.decay)),
        ]
    }

    fn nudge_param(&mut self, param_i: usize, up: bool) -> Result<()> {
        if param_i == 0 {
            self.shape_a.nudge(up)?
        } else if param_i == 1 {
            self.type_a.nudge(up)?
        } else if param_i == 2 {
            self.shape_b.nudge(up)?
        } else if param_i == 3 {
            self.type_b.nudge(up)?
        } else if param_i == 4 {
            self.voice.nudge(up)?
        } else if param_i == 5 {
            self.volume.nudge(up)?
        } else if param_i == 6 {
            self.attack.nudge(up)?
        } else if param_i == 7 {
            self.decay.nudge(up)?
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct WaveChannel {
    steps: [Step<WaveChannelParams>; 16],
}

impl Index<Cursor> for WaveChannel {
    type Output = Step<WaveChannelParams>;

    fn index(&self, index: Cursor) -> &Self::Output {
        &self.steps[index.step_num]
    }
}

impl IndexMut<Cursor> for WaveChannel {
    fn index_mut(&mut self, index: Cursor) -> &mut Self::Output {
        &mut self.steps[index.step_num]
    }
}

impl Index<usize> for WaveChannel {
    type Output = Step<WaveChannelParams>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.steps[index]
    }
}

impl IndexMut<usize> for WaveChannel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.steps[index]
    }
}

impl Channel<WaveChannelParams> for WaveChannel {
    fn get_steps(&self) -> [Step<WaveChannelParams>; 16] {
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
