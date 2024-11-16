use crate::ScreenState;
use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    log::*,
    prelude::*,
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::controls::ControlsPlugin loaded");

        app.init_resource::<NextScreen>()
            .add_systems(Update, gamepad_connections);
        // .add_systems(
        //     Update,
        //     screen_change.run_if(not(in_state(ExitMenuState::Opened))),
        // )
        // .add_systems(Update, gamepad_input);
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub struct NextScreen(Option<ScreenState>);

/// Simple resource to store the ID of the first connected gamepad.
/// We can use it to know which gamepad to use for player input.
#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}

// fn gamepad_input(
//     buttons: Res<ButtonInput<GamepadButton>>,
//     my_gamepad: Option<Res<MyGamepad>>,
//     // keys: Res<ButtonInput<KeyCode>>,
// ) {
//     let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
//         // no gamepad is connected
//         return;
//     };
//
//     // In a real game, the buttons would be configurable, but here we hardcode them
//     let b_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::South,
//     };
//     let a_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::East,
//     };
//     let x_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::North,
//     };
//     let y_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::West,
//     };
//     let up_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadUp,
//     };
//     let down_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadDown,
//     };
//     let left_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadLeft,
//     };
//     let right_button = GamepadButton {
//         gamepad,
//         button_type: GamepadButtonType::DPadRight,
//     };
//
//     // if buttons.just_pressed(b_button) || keys.just_pressed(KeyCode::KeyZ) {
//     if buttons.just_pressed(b_button) {
//         // button just pressed: make the player jump
//         println!("B Button pressed");
//     }
//
//     // if buttons.just_pressed(a_button) || keys.just_pressed(KeyCode::KeyX) {
//     if buttons.just_pressed(a_button) {
//         // button being held down: heal the player
//         println!("A Button pressed");
//     }
//
//     if buttons.just_pressed(x_button) {
//         println!("X Button pressed");
//     }
//
//     if buttons.just_pressed(y_button) {
//         println!("Y Button pressed");
//     }
//
//     if buttons.just_pressed(up_button) {
//         println!("UP Button pressed");
//     }
//
//     if buttons.just_pressed(down_button) {
//         println!("DOWN Button pressed");
//     }
//
//     if buttons.just_pressed(left_button) {
//         println!("LEFT Button pressed");
//     }
//
//     if buttons.just_pressed(right_button) {
//         println!("RIGHT Button pressed");
//     }
// }
