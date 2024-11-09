use crate::{
    controls::{LastAdded, MyGamepad},
    pygame_coms::{DisplayCursor, Phrase, Screen},
    tracker_state::{AllChains, AllPhrases, StateUpdated},
    ScreenState,
};
use bevy::{log::*, prelude::*};

pub struct ChainMenuPlugin;

impl Plugin for ChainMenuPlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::controls::Song loaded");

        app.add_systems(Update, set_phrase.run_if(in_state(ScreenState::EditChain)))
            .add_systems(Update, add_phrase.run_if(in_state(ScreenState::EditChain)))
            .add_systems(Update, movement.run_if(in_state(ScreenState::EditChain)))
            .add_systems(Update, set_select.run_if(in_state(ScreenState::EditChain)))
            .add_systems(
                Update,
                change_phrase.run_if(in_state(ScreenState::EditChain)),
            )
            .add_systems(Update, rm.run_if(in_state(ScreenState::EditChain)))
            // .add_systems(Update, play.run_if(in_state(ScreenState::EditSong)))
            .add_systems(OnEnter(ScreenState::EditChain), set_selected)
            .add_systems(OnEnter(ScreenState::EditChain), set_cursor);
    }
}

fn set_selected(mut display_cursor: ResMut<DisplayCursor>) {
    display_cursor.selected = false;
}

fn set_cursor(mut display_cursor: ResMut<DisplayCursor>) {
    display_cursor.col = 0;
    // state_updated.0 = true;
}

fn set_select(
    mut display_cursor: ResMut<DisplayCursor>,
    mut state_updated: ResMut<StateUpdated>,
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
        state_updated.0 = true;

        display_cursor.selected = !(!buttons.pressed(a_button) && display_cursor.selected)
            || (buttons.pressed(a_button) && !display_cursor.selected);
    }
}

fn change_phrase(
    screen: Res<Screen>,
    phrases: ResMut<AllPhrases>,
    mut chains: ResMut<AllChains>,
    display_cursor: Res<DisplayCursor>,
    mut state_updated: ResMut<StateUpdated>,
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

    if let Screen::EditChain(chain_i) = *screen {
        if let Some(ref mut chain) = chains.0[chain_i]
            && buttons.just_released(up_button)
            && buttons.pressed(a_button)
            && display_cursor.selected
            && !state_updated.0
        {
            if let Some(ref mut phrase) = chain.rows[display_cursor.row].phrase
                && *phrase < 255
            {
                *phrase += 1;
                state_updated.0 = true;
            }
        } else if let Some(ref mut chain) = chains.0[chain_i]
            && buttons.just_released(right_button)
            && buttons.pressed(a_button)
            && display_cursor.selected
            && !state_updated.0
        {
            if let Some(ref mut phrase) = chain.rows[display_cursor.row].phrase
                && *phrase < 255 - 16
            {
                *phrase += 16;
                state_updated.0 = true;
            }
        } else if let Some(ref mut chain) = chains.0[chain_i]
            && buttons.just_released(down_button)
            && buttons.pressed(a_button)
            && display_cursor.selected
            && !state_updated.0
        {
            if let Some(ref mut phrase) = chain.rows[display_cursor.row].phrase
                && *phrase > 0
            {
                *phrase -= 1;
                state_updated.0 = true;
            }
        } else if let Some(ref mut chain) = chains.0[chain_i]
            && buttons.just_released(left_button)
            && buttons.pressed(a_button)
            && display_cursor.selected
            && !state_updated.0
        {
            if let Some(ref mut phrase) = chain.rows[display_cursor.row].phrase
                && *phrase > 15
            {
                *phrase -= 16;
                state_updated.0 = true;
            }
        }
    }

    add_phrase(
        screen,
        chains.into(),
        phrases,
        last_added,
        display_cursor.into(),
    )
}

fn add_phrase(
    screen: Res<Screen>,
    chains: Res<AllChains>,
    mut phrases: ResMut<AllPhrases>,
    mut last_added: ResMut<LastAdded>,
    display_cursor: Res<DisplayCursor>,
) {
    if let Screen::EditChain(chain_i) = *screen {
        if let Some(phrase_i) = chains.0[chain_i].unwrap().rows[display_cursor.row].phrase {
            phrases.0[phrase_i] = Some(Phrase::default());
            last_added.phrase = phrase_i;
        }
    }
}

fn set_phrase(
    mut chains: ResMut<AllChains>,
    phrases: ResMut<AllPhrases>,
    mut display_cursor: ResMut<DisplayCursor>,
    mut state_updated: ResMut<StateUpdated>,
    last_added: ResMut<LastAdded>,
    screen: Res<Screen>,
) {
    if let Screen::EditChain(chain) = *screen
        && chains.0[chain].unwrap().rows[display_cursor.row]
            .phrase
            .is_none()
        && display_cursor.selected
    {
        // error!(
        //     "{:?}",
        //     chains.0[chain].unwrap().rows[display_cursor.row].phrase
        // );
        if let Some(ref mut chain) = chains.0[chain] {
            chain.rows[display_cursor.row].phrase = Some(last_added.phrase);
        }
        // error!(
        //     "{:?}",
        //     chains.0[chain].unwrap().rows[display_cursor.row].phrase
        // );
        display_cursor.selected = true;
        state_updated.0 = true;
    }

    add_phrase(
        screen,
        chains.into(),
        phrases,
        last_added,
        display_cursor.into(),
    )
}

fn movement(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut display_cursor: ResMut<DisplayCursor>,
    mut state_updated: ResMut<StateUpdated>,
    gamepads: Res<Gamepads>,
) {
    if !display_cursor.selected {
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
            state_updated.0 = true;
        }

        if buttons.just_released(down_button) && !buttons.pressed(start_button) {
            let new_row = if display_cursor.row == 15 {
                0
            } else {
                display_cursor.row + 1
            };

            display_cursor.row = new_row;
            state_updated.0 = true;
        }
    }
}

fn rm(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    screen: Res<Screen>,
    display_cursor: Res<DisplayCursor>,
    mut state_updated: ResMut<StateUpdated>,
    mut chains: ResMut<AllChains>,
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

    if let Screen::EditChain(chain) = *screen
        && ((buttons.just_released(a_button) && buttons.pressed(b_button))
            || (buttons.just_released(b_button) && buttons.pressed(a_button)))
    {
        chains.0[chain].unwrap().rows[display_cursor.row].phrase = None;
        state_updated.0 = true;
    }
}
