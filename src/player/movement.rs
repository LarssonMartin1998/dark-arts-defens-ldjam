use crate::player::spawn::Player;
use crate::velocity::Velocity;
use bevy::prelude::*;

pub fn system(keys: Res<ButtonInput<KeyCode>>, query: Query<&mut Velocity, With<Player>>) {
    let column_staggered_colemak_binds =
        [KeyCode::KeyF, KeyCode::KeyR, KeyCode::KeyS, KeyCode::KeyT];
    // let row_staggered_qwerty_binds = [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD];
    // let move_input = construct_input_vector(&keys, row_staggered_qwerty_binds);
    let move_input = construct_input_vector(keys, column_staggered_colemak_binds);
    handle_movement(query, move_input);
}

fn construct_input_vector(keys: Res<ButtonInput<KeyCode>>, binds: [KeyCode; 4]) -> Vec2 {
    let mut move_input = Vec2::ZERO;
    if keys.pressed(binds[0]) {
        move_input.y += 1.0;
    }
    if keys.pressed(binds[1]) {
        move_input.x -= 1.0;
    }
    if keys.pressed(binds[2]) {
        move_input.y -= 1.0;
    }
    if keys.pressed(binds[3]) {
        move_input.x += 1.0;
    }

    move_input = move_input.normalize_or_zero();
    move_input
}

fn handle_movement(mut query: Query<&mut Velocity, With<Player>>, move_input: Vec2) {
    let mut velocity = query.single_mut();
    velocity.0 = move_input;
}
