use super::{traits::Nudgable, InstrumentParams};
use crate::MidiNote;
use anyhow::{bail, Result};

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
    pub fn nudge(&mut self, up: bool, param_num: Option<usize>) -> Result<()> {
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
            self.note.nudge(up)?;
        }

        Ok(())
    }
}
