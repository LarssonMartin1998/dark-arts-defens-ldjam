use bevy::prelude::*;

use crate::player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, player::spawn::system)
            .add_systems(
                Update,
                (
                    player::spawn::animate_sprite,
                ),
            );
    }
}
