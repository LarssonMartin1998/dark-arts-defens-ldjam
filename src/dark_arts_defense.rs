use bevy::prelude::*;

use crate::animation;
use crate::player;
use crate::velocity;

pub struct DarkArtsDefensePlugin;

impl Plugin for DarkArtsDefensePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::plugin::PlayerPlugin).add_systems(
            Update,
            (
                animation::animate_sprite,
                animation::update_animation_visibility,
                animation::debug_change_anim_state,
                animation::handle_anim_state,
                velocity::translate,
                velocity::change_sprite_direction,
            ),
        );
    }
}
