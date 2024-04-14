use bevy::prelude::*;

use crate::player;
use crate::units::unit_types::UnitResource;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitResource::default())
            .add_systems(Startup, player::spawn::system)
            .add_systems(
                Update,
                (player::movement::system, player::summoning::system),
            );
    }
}
