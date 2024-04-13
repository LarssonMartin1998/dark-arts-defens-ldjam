use crate::player::spawn::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub fn system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Query<(&PlayerMovement, &mut Transform, &Children), With<Player>>,
    child_query: Query<&mut Sprite>,
) {
    let column_staggered_colemak_binds =
        [KeyCode::KeyF, KeyCode::KeyR, KeyCode::KeyS, KeyCode::KeyT];
    // let row_staggered_qwerty_binds = [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD];
    // let move_input = construct_input_vector(&keys, row_staggered_qwerty_binds);
    let move_input = construct_input_vector(keys, column_staggered_colemak_binds);
    handle_movement(query, child_query, time, move_input);
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
    mut query: Query<(&PlayerMovement, &mut Transform, &Children), With<Player>>,
    mut child_query: Query<&mut Sprite>,
    time: Res<Time>,
    move_input: Vec2,
) {
    let (movement, mut transform, children) = query.single_mut();
    if move_input.length() > 0.0 {
        for child in children.iter() {
            if let Ok(mut sprite) = child_query.get_mut(*child) {
                if move_input.x != 0.0 {
                    sprite.flip_x = move_input.x < 0.0;
                }
            }
        }
    }

    transform.translation.x += movement.speed * move_input.x * time.delta_seconds();
    transform.translation.y += movement.speed * move_input.y * time.delta_seconds();
}
