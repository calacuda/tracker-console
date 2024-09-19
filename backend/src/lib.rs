use bevy::{a11y::AccessibilityPlugin, prelude::*};
use controls::ControlsPlugin;
use ipc::{gen_ipc, RustIPC, TrackerIPC};
// use loging::logger_init;
use pygame_coms::{Button, InputCMD, RenderCMD};
use pyo3::prelude::*;
use std::{path::PathBuf, thread::spawn, time::Instant};
use tracker_lib::{TrackerConfig, TrackerState};

pub mod base_disaply;
pub mod ipc;
// pub mod loging;
pub mod controls;
pub mod pygame_coms;

#[derive(Clone, Copy, PartialEq, Debug, Resource)]
pub struct ScreenSize(Vec2);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Resource)]
pub struct AssetDir(PathBuf);

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
        let mut config = TrackerConfig::default();
        config.colors.text = [10, 100, 20];
        config.ui.menu.tempo = 1.0 / 8.0;
        config.font.size = vec![30].into();

        app.insert_resource(ControllerInput::new());
        app.insert_resource(io.clone());
        app.insert_resource(config);
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
                                info!("exiting form runner loop becuase of PyGame Exit.");
                                return AppExit::Success;
                            }
                            InputCMD::ButtonPress(button) => ctrl.press(button),
                            InputCMD::ButtonRelease(button) => ctrl.release(button),
                        }
                    }
                }
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
                info!("exiting form runner loop becuase of in program shutdown.");
                return exit;
            }
        }
    };

    runner
}

fn start(io: RustIPC, screen_w: f32, screen_h: f32, asset_dir: PathBuf) {
    info!("start");

    App::new()
        .insert_resource(TrackerState::default())
        .insert_resource(ScreenSize(Vec2 {
            x: screen_w,
            y: screen_h,
        }))
        .insert_resource(AssetDir(asset_dir))
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<WindowPlugin>()
                .disable::<FrameCountPlugin>()
                .disable::<AccessibilityPlugin>(),
        )
        .add_plugins(ControlsPlugin)
        .add_plugins(base_disaply::BaseDisplayPlugin)
        .set_runner(build_runner(io))
        .run();

    info!("goodbye");
}

#[pyfunction]
fn run(screen_w: f32, screen_h: f32, asset_dir: PathBuf) -> PyResult<TrackerIPC> {
    let (rust_input, python_input) = gen_ipc();

    spawn(move || start(rust_input, screen_w, screen_h, asset_dir));

    Ok(python_input)
}

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn tracker_backend(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    // m.add_class::<TrackerIPC>()?;
    m.add_class::<Button>()?;
    m.add_class::<InputCMD>()?;
    m.add_class::<RenderCMD>()?;
    Ok(())
}
