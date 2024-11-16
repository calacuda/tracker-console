use crate::{
    controls::{LastAdded, MyGamepad},
    pygame_coms::{Chain, DisplayCursor, Song},
    tracker_state::{AllChains, StateUpdated},
    ExitMenuState, ScreenState,
};
use bevy::{log::*, prelude::*};
use std::ops::DerefMut;

pub struct SongMenuPlugin;

impl Plugin for SongMenuPlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::controls::Song loaded");

        app.add_systems(
            Update,
            set_chain
                .run_if(in_state(ScreenState::EditSong))
                .run_if(not(in_state(ExitMenuState::Opened))),
        )
        .add_systems(
            Update,
            add_chain
                .run_if(in_state(ScreenState::EditSong))
                .run_if(not(in_state(ExitMenuState::Opened))),
        )
        .add_systems(
            Update,
            movement
                .run_if(in_state(ScreenState::EditSong))
                .run_if(not(in_state(ExitMenuState::Opened))),
        )
        .add_systems(
            Update,
            set_select
                .run_if(in_state(ScreenState::EditSong))
                .run_if(not(in_state(ExitMenuState::Opened))),
        )
        .add_systems(
            Update,
            change_chain
                .run_if(in_state(ScreenState::EditSong))
                .run_if(not(in_state(ExitMenuState::Opened))),
        )
        .add_systems(
            Update,
            rm.run_if(in_state(ScreenState::EditSong))
                .run_if(not(in_state(ExitMenuState::Opened))),
        )
        .add_systems(OnEnter(ScreenState::EditSong), set_selected);
    }
}

fn set_selected(mut display_cursor: ResMut<DisplayCursor>) {
    display_cursor.selected = false;
}

fn set_select(
    mut display_cursor: ResMut<DisplayCursor>,
    mut state_updated: EventWriter<StateUpdated>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let a_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if (!buttons.pressed(a_button) && display_cursor.selected)
        || (buttons.pressed(a_button) && !display_cursor.selected)
    {
        state_updated.send_default();
        display_cursor.selected = !(!buttons.pressed(a_button) && display_cursor.selected)
            || (buttons.pressed(a_button) && !display_cursor.selected);
    }
}

fn change_chain(
    chains: ResMut<AllChains>,
    mut song: ResMut<Song>,
    display_cursor: Res<DisplayCursor>,
    mut state_updated: EventWriter<StateUpdated>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    last_added: ResMut<LastAdded>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let up_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadUp,
    };
    let down_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadDown,
    };
    let left_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };
    let a_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if let Some(ref mut chain) = song.rows[display_cursor.row][display_cursor.col]
        && buttons.just_released(up_button)
        && buttons.pressed(a_button)
        && display_cursor.selected
        // && !state_updated.0
        && *chain < 255
    {
        *chain += 1;
        state_updated.send_default();
    } else if let Some(ref mut chain) = song.rows[display_cursor.row][display_cursor.col]
        && buttons.just_released(right_button)
        && buttons.pressed(a_button)
        && display_cursor.selected
        // && !state_updated.0
        && *chain < 255 - 16
    {
        *chain += 16;
        // state_updated.0 = true;
        state_updated.send_default();
    } else if let Some(ref mut chain) = song.rows[display_cursor.row][display_cursor.col]
        && buttons.just_released(down_button)
        && buttons.pressed(a_button)
        && display_cursor.selected
        // && !state_updated.0
        && *chain > 0
    {
        *chain -= 1;
        // state_updated.0 = true;
        state_updated.send_default();
    } else if let Some(ref mut chain) = song.rows[display_cursor.row][display_cursor.col]
        && buttons.just_released(left_button)
        && buttons.pressed(a_button)
        && display_cursor.selected
        // && !state_updated.0
        && *chain > 15
    {
        *chain -= 16;
        // state_updated.0 = true;
        state_updated.send_default();
    }

    add_chain(chains, song.into(), display_cursor, last_added);
}

fn add_chain(
    mut chains: ResMut<AllChains>,
    song: Res<Song>,
    display_cursor: Res<DisplayCursor>,
    mut last_added: ResMut<LastAdded>,
) {
    if let Some(chain_i) = song.rows[display_cursor.row][display_cursor.col] {
        if chains.0[chain_i].is_none() {
            let mut chain = Chain::default();
            chain.name = chain_i;

            chains.0[chain_i] = Some(chain);
        }

        last_added.deref_mut().chain = chain_i;
    }
}

fn set_chain(
    chains: ResMut<AllChains>,
    mut song: ResMut<Song>,
    mut display_cursor: ResMut<DisplayCursor>,
    mut state_updated: EventWriter<StateUpdated>,
    last_added: ResMut<LastAdded>,
) {
    if song.rows[display_cursor.row][display_cursor.col].is_none() && display_cursor.selected {
        song.rows[display_cursor.row][display_cursor.col] = Some(last_added.chain);
        display_cursor.selected = true;
        // state_updated.0 = true;
        state_updated.send_default();
    }

    add_chain(chains, song.into(), display_cursor.into(), last_added)
}

fn movement(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut display_cursor: ResMut<DisplayCursor>,
    mut state_updated: EventWriter<StateUpdated>,
    gamepads: Res<Gamepads>,
) {
    if display_cursor.selected {
        return;
    }

    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let up_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadUp,
    };
    let down_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadDown,
    };
    let left_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };

    let start_button = if let Some(name) = gamepads.name(gamepad)
        && name.starts_with("PS5")
    {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }
    } else {
        GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Select,
        }
    };

    if buttons.just_released(up_button) && !buttons.pressed(start_button) {
        let new_row = if display_cursor.row == 0 {
            15
        } else {
            display_cursor.row - 1
        };

        display_cursor.row = new_row;
        // state_updated.0 = true;
        state_updated.send_default();
    }

    if buttons.just_released(down_button) && !buttons.pressed(start_button) {
        let new_row = if display_cursor.row == 15 {
            0
        } else {
            display_cursor.row + 1
        };

        display_cursor.row = new_row;
        // state_updated.0 = true;
        state_updated.send_default();
    }

    if buttons.just_released(left_button) && !buttons.pressed(start_button) {
        let new_col = if display_cursor.col == 0 {
            3
        } else {
            display_cursor.col - 1
        };

        display_cursor.col = new_col;
        // state_updated.0 = true;
        state_updated.send_default();
    }

    if buttons.just_released(right_button) && !buttons.pressed(start_button) {
        let new_col = if display_cursor.col == 3 {
            0
        } else {
            display_cursor.col + 1
        };

        display_cursor.col = new_col;
        // state_updated.0 = true;
        state_updated.send_default();
    }
}

fn rm(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    display_cursor: Res<DisplayCursor>,
    mut state_updated: EventWriter<StateUpdated>,
    mut song: ResMut<Song>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let b_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };
    let a_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    if (buttons.just_released(a_button) && buttons.pressed(b_button))
        || (buttons.just_released(b_button) && buttons.pressed(a_button))
    {
        song.rows[display_cursor.row][display_cursor.col] = None;
        // state_updated.0 = true;
        state_updated.send_default();
    }
}
