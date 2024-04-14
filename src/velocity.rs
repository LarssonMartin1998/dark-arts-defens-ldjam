use bevy::prelude::*;

use crate::{movement::Movement, units::health::Health};

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub fn translate(
    time: Res<Time>,
    mut query: Query<(&Velocity, &Movement, &Health, &mut Transform)>,
) {
    for (velocity, movement, health, mut transform) in query.iter_mut() {
        if health.is_dead() {
            continue;
        }

        transform.translation.x += velocity.0.x * movement.speed * time.delta_seconds();
        transform.translation.y += velocity.0.y * movement.speed * time.delta_seconds();
    }
}

pub fn change_sprite_direction(
    query: Query<(&Velocity, &Children)>,
    mut child_query: Query<&mut Sprite>,
) {
    for (velocity, children) in query.iter() {
        for child in children.iter() {
            if let Ok(mut sprite) = child_query.get_mut(*child) {
                if velocity.0.x != 0.0 {
                    sprite.flip_x = velocity.0.x < 0.0;
                }
            }
        }
    }
}
