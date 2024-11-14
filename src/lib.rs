#![feature(let_chains)]
use crate::config::ui::{get_config, TrackerConfig};
use bevy::{a11y::AccessibilityPlugin, log::LogPlugin, prelude::*};
use chain_menu::ChainMenuPlugin;
use config::ui::{ColorsConfig, FontConfig, MenuUiConf, TabUiConf, UiConfig};
use controls::ControlsPlugin;
use ipc::{gen_ipc, RustIPC, TrackerIPC};
use phrase_menu::PhraseMenuPlugin;
use pygame_coms::{
    Button, Chain, ChainRow, InputCMD, Instrument, Phrase, PhraseRow, PlaybackCursor, Screen,
    ScreenData, Song, SongRow, State, TrackerCommand,
};
use pyo3::prelude::*;
use song_menu::SongMenuPlugin;
use std::{thread::spawn, time::Instant};
use tracker_state::TrackerStatePlugin;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenState {
    #[default]
    EditSong,
    EditChain,
    EditPhrase,
    EditInsts,
    PlaySynth,
    Settings,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayingState {
    Playing,
    #[default]
    NotPlaying,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExitMenuState {
    Opened,
    #[default]
    Closed,
}

pub mod chain_menu;
pub mod config;
pub mod controls;
pub mod ipc;
pub mod phrase_menu;
pub mod pygame_coms;
pub mod song_menu;
pub mod tracker_state;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Resource, Component)]
pub struct ControllerInput {
    pub just_pressed: Vec<Button>,
    pub held: Vec<(Button, Instant)>,
    pub just_released: Vec<Button>,
}

impl ControllerInput {
    fn new() -> Self {
        Self {
            just_pressed: Vec::with_capacity(15),
            held: Vec::with_capacity(15),
            just_released: Vec::with_capacity(15),
        }
    }

    fn press(&mut self, button: Button) {
        self.just_pressed.push(button);
        self.held.push((button, Instant::now()));
    }

    fn release(&mut self, button: Button) {
        // self.newly_pressed.push(button);
        // self.held.retain(|(b, inst)| button != *b);
        self.just_released.push(button);
    }

    fn cleanup(&mut self) {
        self.held
            .retain(|(b, _inst)| !self.just_released.contains(b));
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

fn build_runner(io: RustIPC) -> impl FnMut(App) -> AppExit {
    let runner = move |mut app: App| -> AppExit {
        app.insert_resource(ControllerInput::new());
        app.insert_resource(io.clone());
        // app.insert_resource(config);
        app.finish();
        app.cleanup();

        loop {
            if io.len() > 0 {
                let world = app.world_mut();

                if let Some(mut ctrl) = world.get_resource_mut::<ControllerInput>() {
                    // recv loop
                    while let Some(msg) = io.recv_msg() {
                        // debug!("{msg:?}");
                        debug!("msg: {msg:?}");

                        match msg {
                            InputCMD::Exit() => {
                                info!("exiting from runner loop becuase of PyGame Exit.");
                                return AppExit::Success;
                            }
                            InputCMD::ButtonPress(button) => ctrl.press(button),
                            InputCMD::ButtonRelease(button) => ctrl.release(button),
                        }
                    }

                    // println!("controller found, msg processed")
                }
                // else {
                //     println!("no controller input resource found");
                // }
            }

            app.update();

            {
                let world = app.world_mut();
                // world.insert_resource(ControllerInput::new());
                world
                    .get_resource_mut::<ControllerInput>()
                    .map(|mut ctrl| ctrl.cleanup());
            }

            if let Some(exit) = app.should_exit() {
                info!("exiting from runner loop becuase of a program shutdown.");
                return exit;
            }
        }
    };

    runner
}

fn start(io: RustIPC) {
    info!("start");

    App::new()
        // .insert_resource(TrackerState::default())
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<WindowPlugin>()
                .disable::<FrameCountPlugin>()
                .disable::<AccessibilityPlugin>()
                .set(LogPlugin {
                    // filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                    // #[cfg(debug_assertions)]
                    level: bevy::log::Level::DEBUG,
                    // #[cfg(not(debug_assertions))]
                    // level: bevy::log::Level::INFO,
                    ..Default::default()
                }),
        )
        .add_plugins(ControlsPlugin)
        // .add_plugins(base_display::BaseDisplayPlugin)
        .add_plugins(TrackerStatePlugin)
        .add_plugins(SongMenuPlugin)
        .add_plugins(ChainMenuPlugin)
        .add_plugins(PhraseMenuPlugin)
        // .insert_state(ScreenData::Song)
        .init_state::<ScreenState>()
        .init_state::<PlayingState>()
        .init_state::<ExitMenuState>()
        .set_runner(build_runner(io))
        .run();

    info!("goodbye");
}

#[pyfunction]
fn run() -> PyResult<TrackerIPC> {
    let (rust_input, python_input) = gen_ipc();

    spawn(move || start(rust_input));

    Ok(python_input)
}

/// A Python module implemented in Rust.
#[pymodule]
fn tracker_backend(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(get_config, m)?)?;

    m.add_class::<TrackerCommand>()?;
    m.add_class::<Button>()?;
    m.add_class::<InputCMD>()?;
    m.add_class::<Instrument>()?;
    m.add_class::<PhraseRow>()?;
    m.add_class::<Phrase>()?;
    m.add_class::<ChainRow>()?;
    m.add_class::<Chain>()?;
    m.add_class::<SongRow>()?;
    m.add_class::<Song>()?;
    m.add_class::<Screen>()?;
    m.add_class::<PlaybackCursor>()?;
    m.add_class::<State>()?;
    m.add_class::<ScreenData>()?;
    m.add_class::<TrackerConfig>()?;
    m.add_class::<TrackerCommand>()?;
    m.add_class::<FontConfig>()?;
    m.add_class::<ColorsConfig>()?;
    m.add_class::<UiConfig>()?;
    m.add_class::<MenuUiConf>()?;
    m.add_class::<TabUiConf>()?;
    // m.add_class::<>()?;
    // m.add_class::<>()?;
    // m.add_class::<>()?;
    // m.add_class::<>()?;
    // m.add_class::<>()?;
    // m.add_class::<>()?;

    Ok(())
}
