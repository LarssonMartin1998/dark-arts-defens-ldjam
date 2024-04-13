use bevy::prelude::*;

use crate::player;

pub struct DarkArtsDefensePlugin;

impl Plugin for DarkArtsDefensePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::plugin::PlayerPlugin);
    }
}
