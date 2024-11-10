use crate::{
    controls::{LastAdded, MyGamepad},
    pygame_coms::{DisplayCursor, Index, Instrument, Note, Screen},
    tracker_state::{AllInstruments, AllPhrases, StateUpdated},
    ScreenState,
};
use bevy::{log::*, prelude::*};
use std::ops::DerefMut;

#[derive(Debug, Clone, Default, Resource)]
struct PhraseIndex(Index);

pub struct PhraseMenuPlugin;

impl Plugin for PhraseMenuPlugin {
    fn build(&self, app: &mut App) {
        debug!("tracker_backend::controls::Phrase Menu Plugin loaded");

        app
            // .add_systems(Update, set_phrase.run_if(in_state(ScreenState::EditPhrase)))
            .init_resource::<PhraseIndex>()
            .add_event::<EnterSelect>()
            .add_event::<EditNote>()
            .add_event::<EditInst>()
            .add_event::<EditCmd>()
            .add_systems(Update, edit_note.run_if(in_state(ScreenState::EditPhrase)))
            .add_systems(Update, edit_inst.run_if(in_state(ScreenState::EditPhrase)))
            .add_systems(Update, edit_cmd.run_if(in_state(ScreenState::EditPhrase)))
            .add_systems(Update, movement.run_if(in_state(ScreenState::EditPhrase)))
            .add_systems(Update, set_select.run_if(in_state(ScreenState::EditPhrase)))
            .add_systems(
                Update,
                change_entry.run_if(in_state(ScreenState::EditPhrase)),
            )
            .add_systems(Update, rm.run_if(in_state(ScreenState::EditPhrase)))
            // .add_systems(Update, play.run_if(in_state(ScreenState::EditSong)))
            .add_systems(OnEnter(ScreenState::EditPhrase), set_phrase_index)
            .add_systems(OnEnter(ScreenState::EditPhrase), set_selected)
            .add_systems(OnEnter(ScreenState::EditPhrase), set_cursor);
        // .add_systems(OnEnter(ScreenState::EditPhrase), log_phrase_data)
        // .add_systems(OnEnter(ScreenState::EditPhrase), update_display);
    }
}

#[derive(Event, Debug, Default)]
struct EnterSelect;

#[derive(Event, Debug)]
struct EditNote {
    // row: Index,
    delta: i8,
}

impl Default for EditNote {
    fn default() -> Self {
        Self { delta: 0 }
    }
}

#[derive(Event, Debug)]
struct EditInst {
    delta: i8,
}

impl Default for EditInst {
    fn default() -> Self {
        Self { delta: 0 }
    }
}

#[derive(Event, Debug, Default)]
struct EditCmd {
    /// true when changing the command, false when changing the args
    change_cmd: bool,
    /// true when shifting up, false when shifting down
    up: bool,
}

// fn log_phrase_data(phrases: Res<AllPhrases>, screen: Res<Screen>) {
//     if let Screen::EditPhrase(phrase_i) = *screen {
//         debug!(
//             "editing phrase numer {phrase_i}: it has data: {:?}.",
//             phrases.0[phrase_i]
//         );
//     }
// }

// fn update_display(mut state_updated: ResMut<StateUpdated>) {
//     state_updated.0 = true;
// }

fn set_phrase_index(mut phrase_index: ResMut<PhraseIndex>, screen: Res<Screen>) {
    if let Screen::EditPhrase(phrase_i) = *screen {
        phrase_index.0 = phrase_i;
    }
}

fn set_selected(mut display_cursor: ResMut<DisplayCursor>) {
    display_cursor.selected = false;
}

fn set_cursor(mut display_cursor: ResMut<DisplayCursor>) {
    if display_cursor.col == 3 {
        display_cursor.col = 2;
    }
}

fn set_select(
    mut display_cursor: ResMut<DisplayCursor>,
    mut state_updated: EventWriter<StateUpdated>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    // mut select_event: EventWriter<EnterSelect>,
    mut edit_note_event: EventWriter<EditNote>,
    mut edit_inst_event: EventWriter<EditInst>,
    mut edit_cmd_event: EventWriter<EditCmd>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    let a_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };

    let b_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };

    if ((!buttons.pressed(a_button) && display_cursor.selected)
        || (buttons.pressed(a_button) && !display_cursor.selected))
        && !buttons.pressed(b_button)
    {
        // state_updated.0 = true;
        state_updated.send_default();
        display_cursor.selected = !(!buttons.pressed(a_button) && display_cursor.selected)
            || (buttons.pressed(a_button) && !display_cursor.selected);

        if display_cursor.col == 0 {
            edit_note_event.send_default();
        } else if display_cursor.col == 1 {
            edit_inst_event.send_default();
        } else if display_cursor.col == 2 {
            edit_cmd_event.send_default();
        }
    }
}

fn change_entry(
    phrases: Res<AllPhrases>,
    display_cursor: Res<DisplayCursor>,
    // state_updated: EventWriter<StateUpdated>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    phrase_index: Res<PhraseIndex>,
    mut edit_note_event: EventWriter<EditNote>,
    mut edit_inst_event: EventWriter<EditInst>,
    mut edit_cmd_event: EventWriter<EditCmd>,
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
    let b_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };

    let phrase_i = phrase_index.0;

    if phrases.0[phrase_i].is_none() {
        error!("trying to view a non existant phrase_tab.");
        return;
    }

    if buttons.just_released(up_button)
        && buttons.pressed(a_button)
        && !buttons.pressed(b_button)
        && display_cursor.selected
    // && !state_updated.0
    {
        if display_cursor.col == 0 {
            // send edit note event
            edit_note_event.send(EditNote { delta: 1 });
        } else if display_cursor.col == 1 {
            // send edit instrument event
            edit_inst_event.send(EditInst { delta: 1 });
        } else if display_cursor.col == 2 {
            // send edit command event
            edit_cmd_event.send(EditCmd {
                change_cmd: true,
                up: true,
            });
        } else {
            error!("column set to value that is too high for the phrases tab.");
        }
    }

    if buttons.just_released(right_button)
        && buttons.pressed(a_button)
        && !buttons.pressed(b_button)
        && display_cursor.selected
    // && !state_updated.0
    {
        if display_cursor.col == 0 {
            // send edit note event
            edit_note_event.send(EditNote { delta: 12 });
        } else if display_cursor.col == 1 {
            // send edit instrument event
            edit_inst_event.send(EditInst { delta: 16 });
        } else if display_cursor.col == 2 {
            // send edit command event
            edit_cmd_event.send(EditCmd {
                change_cmd: false,
                up: true,
            });
        } else {
            error!("column set to value that is too high for the phrases tab.");
        }
    }

    if buttons.just_released(down_button)
        && buttons.pressed(a_button)
        && !buttons.pressed(b_button)
        && display_cursor.selected
    // && !state_updated.0
    {
        if display_cursor.col == 0 {
            // send edit note event
            edit_note_event.send(EditNote { delta: -1 });
        } else if display_cursor.col == 1 {
            // send edit instrument event
            edit_inst_event.send(EditInst { delta: -1 });
        } else if display_cursor.col == 2 {
            // send edit command event
            edit_cmd_event.send(EditCmd {
                change_cmd: true,
                up: false,
            });
        } else {
            error!("column set to value that is too high for the phrases tab.");
        }
    }

    if buttons.just_released(left_button)
        && buttons.pressed(a_button)
        && !buttons.pressed(b_button)
        && display_cursor.selected
    // && !state_updated.0
    {
        if display_cursor.col == 0 {
            // send edit note event
            edit_note_event.send(EditNote { delta: -12 });
        } else if display_cursor.col == 1 {
            // send edit instrument event
            edit_inst_event.send(EditInst { delta: -16 });
        } else if display_cursor.col == 2 {
            // send edit command event
            edit_cmd_event.send(EditCmd {
                change_cmd: false,
                up: false,
            });
        } else {
            error!("column set to value that is too high for the phrases tab.");
        }
    }
}

fn edit_note(
    mut phrases: ResMut<AllPhrases>,
    mut last_added: ResMut<LastAdded>,
    display_cursor: Res<DisplayCursor>,
    mut events: EventReader<EditNote>,
    mut state_updated: EventWriter<StateUpdated>,
    phrase_index: Res<PhraseIndex>,
) {
    let phrase_i = phrase_index.0;

    for ev in events.read() {
        if let Some(Some(ref mut phrase)) = phrases.0.get_mut(phrase_i) {
            if let Some(ref mut note) = phrase.rows[display_cursor.row].note {
                if (ev.delta > 0 && ev.delta as Note <= 128 - *note)
                    || (ev.delta < 0 && (ev.delta.abs() as Note) <= *note)
                {
                    if ev.delta > 0 {
                        *note += ev.delta as Note;
                    } else {
                        *note -= ev.delta.abs() as Note;
                    }
                    last_added.note = *note;
                    // state_updated.0 = true;
                    state_updated.send_default();
                } else {
                    warn!("not changing note.");
                }
            } else {
                info!("adding MIDI note {}.", last_added.note);
                phrase.rows[display_cursor.row].note = Some(last_added.note);
                // state_updated.0 = true;
                state_updated.send_default();
            }
        } else {
            error!("attempting to edit a note phrase {phrase_i}, which does not exist.");
        }
    }
}

fn edit_inst(
    mut phrases: ResMut<AllPhrases>,
    mut instruments: ResMut<AllInstruments>,
    mut last_added: ResMut<LastAdded>,
    display_cursor: Res<DisplayCursor>,
    mut events: EventReader<EditInst>,
    mut state_updated: EventWriter<StateUpdated>,
    phrase_index: Res<PhraseIndex>,
) {
    let phrase_i = phrase_index.0;

    for ev in events.read() {
        if let Some(Some(ref mut phrase)) = phrases.0.get_mut(phrase_i) {
            if let Some(ref mut inst) = phrase.rows[display_cursor.row].instrument {
                if (ev.delta > 0 && ev.delta as Index <= 128 - *inst)
                    || (ev.delta < 0 && (ev.delta.abs() as Index) <= *inst)
                    || ev.delta == 0
                {
                    if ev.delta > 0 {
                        *inst += ev.delta as Index;
                    } else {
                        *inst -= ev.delta.abs() as Index;
                    }
                    last_added.instrument = *inst;
                    // state_updated.0 = true;
                    state_updated.send_default();

                    if instruments.0.get(*inst).is_none() {
                        for _ in instruments.0.len()..*inst {
                            instruments.0.push(None);
                        }

                        debug!("adding instrument at location: {}", instruments.0.len());
                        instruments.0.push(Some(Instrument::new(*inst)));
                    } else if let Some(instrument) = instruments.0.get(*inst)
                        && instrument.is_none()
                    {
                        instruments.0[*inst] = Some(Instrument::new(*inst));
                    }
                } else {
                    warn!("not changing instrument.");
                }
            } else {
                phrase.rows[display_cursor.row].instrument = Some(last_added.instrument);
                // state_updated.0 = true;
                state_updated.send_default();
            }
        } else {
            error!("attempting to edit an instruemnt in phrase {phrase_i}, which does not exist.");
        }
    }
}

fn edit_cmd(
    mut phrases: ResMut<AllPhrases>,
    mut last_added: ResMut<LastAdded>,
    display_cursor: Res<DisplayCursor>,
    mut events: EventReader<EditCmd>,
    mut state_updated: EventWriter<StateUpdated>,
    phrase_index: Res<PhraseIndex>,
) {
    let phrase_i = phrase_index.0;

    for _ev in events.read() {
        debug!("editing of commands");

        if let Some(Some(ref mut phrase)) = phrases.0.get_mut(phrase_i) {
            if let Some(ref mut _cmd) = phrase.rows[display_cursor.row].command {
                // if (ev.delta > 0 && ev.delta as Note <= 128 - *note)
                //     || (ev.delta < 0 && (ev.delta.abs() as Note) < *note)
                // {
                //     if ev.delta > 0 {
                //         *note += ev.delta as Note;
                //     } else {
                //         *note -= ev.delta.abs() as Note;
                //     }
                //     last_added.note = *note;
                //     state_updated.0 = true;
                // } else {
                //     warn!("not changing note.");
                // }
                warn!("TODO: edit command");
            } else {
                phrase.rows[display_cursor.row].command = Some(last_added.command);
                // state_updated.0 = true;
                state_updated.send_default();
            }
        } else {
            error!("attempting to edit a note phrase {phrase_i}, which does not exist.");
        }
    }
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
            2
        } else {
            display_cursor.col - 1
        };

        display_cursor.col = new_col;
        // state_updated.0 = true;
        state_updated.send_default();
    }

    if buttons.just_released(right_button) && !buttons.pressed(start_button) {
        let new_col = if display_cursor.col == 2 {
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
    phrase_index: Res<PhraseIndex>,
    display_cursor: Res<DisplayCursor>,
    mut state_updated: EventWriter<StateUpdated>,
    mut phrases: ResMut<AllPhrases>,
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

    // if let Screen::EditChain(chain) = *screen
    if let Some(ref mut phrase) = phrases.deref_mut().0[phrase_index.0]
        && ((buttons.just_released(a_button) && buttons.pressed(b_button))
            || (buttons.just_released(b_button) && buttons.pressed(a_button))
            || (buttons.just_released(a_button) && buttons.just_released(b_button))
            || (buttons.just_pressed(a_button) && buttons.just_pressed(b_button)))
    {
        if display_cursor.col == 0 {
            phrase.rows[display_cursor.row].note = None;
        } else if display_cursor.col == 1 {
            phrase.rows[display_cursor.row].instrument = None;
        } else if display_cursor.col == 2 {
            phrase.rows[display_cursor.row].command = None;
        } else {
            error!(
                "column index was {}, this is too high for the phrases screen.",
                display_cursor.col
            );
        }

        warn!("rming something");

        // state_updated.0 = true;
        state_updated.send_default();
    }
    // else {
    //     warn!("not rming");
    // }
}
