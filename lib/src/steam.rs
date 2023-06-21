use bevy::prelude::Res;
use bevy_steamworks::*;

pub fn init_steam_input(client: Res<Client>) {
    let input = client.input();
    input.init(false);

    let game_action_set = input.get_action_set_handle("InGameControls");

    let up_action = input.get_digital_action_handle("up");
    let down_action = input.get_digital_action_handle("down");
    let left_action = input.get_digital_action_handle("left");
    let right_action = input.get_digital_action_handle("right");

    let select_cell_action = input.get_digital_action_handle("select_cell");
    let note_action = input.get_digital_action_handle("note");
    let hint_action = input.get_digital_action_handle("hint");
    let exit_action = input.get_digital_action_handle("exit");

    let select_number_direction = input.get_analog_action_handle("select_number");

    let menu_action_set = input.get_action_set_handle("MenuControls");
    let menu_up_action = input.get_digital_action_handle("menu_up");
    let menu_down_action = input.get_digital_action_handle("menu_down");
    let menu_select_action = input.get_digital_action_handle("menu_select");
    let menu_exit_action = input.get_digital_action_handle("menu_exit");
}
