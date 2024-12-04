use super::{Hoverable, Nudgable, Param};
use anyhow::{bail, ensure, Result};
use std::fmt::Display;

pub mod noise;
pub mod pulse;
pub mod pulse_sweep;
pub mod wave;

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
