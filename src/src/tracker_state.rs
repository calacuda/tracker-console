use crate::config::ui::Bpm;
use bevy::{log::*, prelude::*};

pub struct TrackerStatePlugin;

#[derive(Debug, Event, Default)]
pub struct StateUpdated;

impl Plugin for TrackerStatePlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::base_display::TrackerStatePlugin loaded");

        app
            // .insert_resource(StateUpdated(true))
            .add_event::<StateUpdated>()
            .insert_resource(Tempo(120));
    }
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Resource)]
pub struct Tempo(pub Bpm);
