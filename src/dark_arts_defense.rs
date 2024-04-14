use bevy::prelude::*;

use crate::ai;
use crate::animation;
use crate::enemies;
use crate::player;
use crate::units::acolyte;
use crate::velocity;

pub struct DarkArtsDefensePlugin;

impl Plugin for DarkArtsDefensePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::plugin::PlayerPlugin,
            enemies::plugin::EnemyPlugin,
            ai::plugin::AiPlugin,
        ))
        .add_systems(
            Update,
            (
                animation::animate_sprite,
                animation::update_animation_visibility,
                animation::handle_anim_state,
                velocity::translate,
                velocity::change_sprite_direction,
                acolyte::acolyte_mana_giver,
            ),
        );
    }
}
