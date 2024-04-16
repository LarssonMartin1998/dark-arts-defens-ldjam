use bevy::prelude::*;

use crate::ai;
use crate::animation;
use crate::enemies;
use crate::gamestate;
use crate::player;
use crate::ui;
use crate::units::acolyte;
use crate::velocity;
use rand::{rngs::StdRng, SeedableRng};

#[derive(Resource)]
pub struct RandomSeed(pub StdRng);

#[derive(Event)]
pub enum GameEvent {
    StartGame,
    GameOver,
    IncreaseScore,
}

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
            .add_event::<GameEvent>()
            .add_systems(Startup, gamestate::init_game_system)
            .add_systems(
                Update,
                (
                    gamestate::start_game_system,
                    gamestate::game_over_system,
                    gamestate::update_score_system,
                    animation::animation_state_machine,
                    animation::update_animation_visibility,
                    animation::animate_sprite,
                    velocity::translate,
                    acolyte::acolyte_mana_giver,
                ),
            );
    }
}
