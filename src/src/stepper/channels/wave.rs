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
pub struct WaveChannel {}

// impl Channel for WaveChannel {}
