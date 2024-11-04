use crate::{ipc::RustIPC, pygame_coms::RenderCMD, ScreenSize};
use bevy::prelude::*;
use log::{debug, info};
use std::ops::{Deref, DerefMut};
use tracker_lib::{Bpm, MidiNote, TrackerConfig, TrackerState, N_CHANNELS};

pub struct BaseDisplayPlugin;

#[derive(Default, Resource)]
struct MenuMemo {
    // tempo_h: f64,
    tempo_bpm: Bpm,
    notes: [Option<MidiNote>; N_CHANNELS],
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
        tempo_bar(menu_left_most, screen_size, state.tempo, &io, &config);
        memo.deref_mut().tempo_bpm = state.tempo;
    }

    for i in 0..N_CHANNELS {
        let playing = state.deref().get_playing(i);

        if memo.deref().notes[i] != playing {
            debug!("rendering note echange on channel {i}");
            note_display(menu_left_most, screen_size, playing, i, &io, &config);
            memo.deref_mut().notes[i] = playing;
        }
    }
}

fn note_display(
    left_most: f32,
    screen_size: Vec2,
    playing: Option<MidiNote>,
    i: usize,
    io: &Res<RustIPC>,
    config: &TrackerConfig,
) {
    let menu_width = (screen_size.x - left_most) as f64;
    let display_width = menu_width / (N_CHANNELS as f64);
    let middle_y = ((config.ui.menu.tempo * screen_size.y as f64)
        + (config.ui.menu.note_display * screen_size.y as f64))
        / 2.0;
    let middle_x = display_width * i as f64 + (display_width / 2.0) + left_most as f64;

    let ancor = [
        // display_width * i as f64 + (display_width / 2.0) + left_most as f64,
        middle_x, middle_y,
    ];

    let size = [
        // width
        display_width,
        // height
        (config.ui.menu.note_display * screen_size.y as f64)
            - (config.ui.menu.tempo * screen_size.y as f64),
    ];

    let box_cmd = RenderCMD::Rect {
        ancor,
        size,
        // fill_color: config.colors.back_ground,
        fill_color: [0, 255 - (255 as f32 / i as f32) as u8, 0],
        center: true,
    };

    if let Err(e) = io.deref().send_msg(box_cmd) {
        error!("could not send note background box render command to python {e}");
    }

    if let Some(note) = playing {
        let text_cmd = RenderCMD::Text {
            ancor,
            color: config.colors.text,
            text: format!("{note}"),
            font_size: config.font.size[0],
            center: true,
        };

        if let Err(e) = io.deref().send_msg(text_cmd) {
            error!("could not send note text render command to python {e}");
        }
    }
}

fn tempo_bar(
    left_most: f32,
    screen_size: Vec2,
    tempo: u8,
    io: &Res<RustIPC>,
    config: &TrackerConfig,
) {
    let msg = format!("Tempo: {tempo: >3} BPM");

    let color = config.colors.text;

    let cmd = RenderCMD::Text {
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
