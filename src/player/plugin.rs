use bevy::prelude::*;

use crate::player;
use crate::units::unit_types::UnitResource;

#[derive(Resource)]
pub struct AnimTimer(pub Timer);
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(UnitResource::default())
            .add_systems(Startup, player::spawn::system)
            .add_systems(
                Update,
                (
                    player::movement::system,
                    player::spawn::animate_sprite,
                    player::spawn::update_animation_visibility,
                    player::spawn::change_animation_state,
                    player::summoning::system,
                ),
            );
    }
}
