use bevy::prelude::*;

use crate::ai;
use crate::animation;
use crate::enemies;
use crate::player;
use crate::ui;
use crate::units::acolyte;
use crate::velocity;
use rand::{rngs::StdRng, SeedableRng};

#[derive(Resource)]
pub struct RandomSeed(pub StdRng);

pub struct DarkArtsDefensePlugin;

impl Plugin for DarkArtsDefensePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RandomSeed(StdRng::seed_from_u64(12345123454321_u64)))
            .add_plugins((
            player::plugin::PlayerPlugin,
            enemies::plugin::EnemyPlugin,
            ai::plugin::AiPlugin,
            ui::plugin::UiPlugin,
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
