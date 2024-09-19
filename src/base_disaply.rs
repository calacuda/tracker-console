use crate::{ipc::RustIPC, ScreenSize};
use bevy::prelude::*;
use log::info;
use std::ops::{Deref, DerefMut};
use tracker_lib::{Bpm, TrackerConfig, TrackerState};

pub struct BaseDisplayPlugin;

#[derive(Default, Resource)]
struct MenuMemo {
    // tempo_h: f64,
    tempo_bpm: Bpm,
}

impl Plugin for BaseDisplayPlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::base_display::BaseDisplayPlugin loaded");

        app.insert_resource(MenuMemo::default())
            .add_systems(Update, right_hand_menu);
    }
}

fn right_hand_menu(
    screen_size: Res<ScreenSize>,
    config: Res<TrackerConfig>,
    state: Res<TrackerState>,
    io: Res<RustIPC>,
    mut memo: ResMut<MenuMemo>,
) {
    let screen_size = screen_size.deref().0;
    let screen_w = screen_size.x;
    let menu_left_most = screen_w - (screen_w / 3.0);
    // let state = state.deref();
    let config = config.deref();

    if memo.deref().tempo_bpm != state.tempo {
        info!("rendering tempo");
        tempo_bar(menu_left_most, screen_size, state.tempo, io, &config);
        memo.deref_mut().tempo_bpm = state.tempo;
    }
}

fn tempo_bar(
    left_most: f32,
    screen_size: Vec2,
    tempo: u8,
    io: Res<RustIPC>,
    config: &TrackerConfig,
) {
    let msg = format!("Tempo: {tempo: >3} BPM");

    let color = config.colors.text;

    let cmd = crate::pygame_coms::RenderCMD::Text {
        ancor: [
            (left_most + screen_size.x) as f64 / 2.0,
            (config.ui.menu.tempo * screen_size.y as f64) / 2.0,
        ],
        color,
        text: msg,
        font_size: config.font.size[0],
        center: true,
    };

    if let Err(e) = io.deref().send_msg(cmd) {
        error!("could not send render command to python {e}");
    }
}
