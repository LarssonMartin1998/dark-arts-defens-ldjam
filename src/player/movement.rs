use crate::velocity::Velocity;
use bevy::prelude::*;

use super::plugin::Player;

const WINDOW_BOUNDS_OFFSET: f32 = 96.0;

pub fn system(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<(&mut Velocity, &Transform), With<Player>>,
    window_query: Query<&Window>,
) {
    // let column_staggered_colemak_binds =
    //     [KeyCode::KeyF, KeyCode::KeyR, KeyCode::KeyS, KeyCode::KeyT];
    // let move_input = construct_input_vector(keys, column_staggered_colemak_binds);
    let row_staggered_qwerty_binds = [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD];
    let move_input = construct_input_vector(keys, row_staggered_qwerty_binds);
    handle_movement(query, window_query, move_input);
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

fn handle_movement(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    window_query: Query<&Window>,
    move_input: Vec2,
) {
    let window = window_query.single();
    let window_bounds = Vec2::new(
        window.width() - WINDOW_BOUNDS_OFFSET,
        window.height() - WINDOW_BOUNDS_OFFSET,
    ) * 0.5;

    for (mut velocity, transform) in query.iter_mut() {
        velocity.0 = move_input;

        if (transform.translation.x >= window_bounds.x && velocity.0.x > 0.0)
            || (transform.translation.x <= -window_bounds.x && velocity.0.x < 0.0)
        {
            velocity.0.x = 0.0;
        }

        if (transform.translation.y >= window_bounds.y && velocity.0.y > 0.0)
            || (transform.translation.y <= -window_bounds.y && velocity.0.y < 0.0)
        {
            velocity.0.y = 0.0;
        }
    }
}
