use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub fn translate(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
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
