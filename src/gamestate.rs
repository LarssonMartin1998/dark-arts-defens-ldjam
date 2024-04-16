use bevy::prelude::*;

use crate::animation::{spawn_animated_children, AnimatedChildSpawnParams, AnimationType};
use crate::mana::Mana;
use crate::movement::Movement;
use crate::player::plugin::Player;
use crate::units::health::Health;
use crate::units::unit_types::UnitBundle;
use crate::{dark_arts_defense::GameEvent, enemies::enemy_spawner::EnemySpawner};

#[derive(Component, Default)]
pub struct Cleanup;

#[derive(Component)]
pub struct GameState {
    pub game_over: bool,
    pub show_end_timer: Timer,
    pub score: u32,
    pub end_screen_active: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            game_over: false,
            show_end_timer: Timer::from_seconds(5.0, TimerMode::Once),
            score: 0,
            end_screen_active: false,
        }
    }
}

pub fn init_game_system(mut commands: Commands, mut events: EventWriter<GameEvent>) {
    commands.spawn(Camera2dBundle::default());
    events.send(crate::dark_arts_defense::GameEvent::StartGame);
}

pub fn game_over_system(
    time: Res<Time>,
    query: Query<&Health, With<Player>>,
    mut game_state_query: Query<&mut GameState>,
) {
    if let Some(health) = query.iter().next() {
        if health.is_dead() {
            for mut state in game_state_query.iter_mut() {
                state.game_over = true;
                state.show_end_timer.tick(time.delta());
                if state.show_end_timer.just_finished() {
                    state.end_screen_active = true;
                }
            }
        }
    }
}

pub fn update_score_system(
    mut event_reader: EventReader<GameEvent>,
    mut query: Query<&mut GameState>,
) {
    for event in event_reader.read() {
        if let GameEvent::IncreaseScore = event {
            for mut state in query.iter_mut() {
                if !state.game_over {
                    state.score += 10;
                }
            }
        }
    }
}

pub fn start_game_system(
    mut commands: Commands,
    mut event_reader: EventReader<GameEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cleanup_char_query: Query<Entity, With<Cleanup>>,
) {
    for event in event_reader.read() {
        if let GameEvent::StartGame = event {
            cleanup_game_system(&mut commands, &cleanup_char_query);

            commands.spawn((GameState::default(), Cleanup {}));
            commands.spawn((EnemySpawner {}, Cleanup {}));

            commands
                .spawn((
                    UnitBundle {
                        movement: Movement { speed: 150.0 },
                        transform: Transform::from_scale(Vec3::splat(2.0)),
                        ..default()
                    },
                    Player,
                    Mana {
                        current_mana: 100,
                        max_mana: 100,
                    },
                ))
                .with_children(|parent| {
                    let children_params: Vec<AnimatedChildSpawnParams> = [
                        (
                            "player/player_idle.png",
                            Vec2::new(96.0, 96.0),
                            (50, 1),
                            49,
                            AnimationType::Idle,
                            true,
                            false,
                        ),
                        (
                            "player/player_walk.png",
                            Vec2::new(96.0, 96.0),
                            (10, 1),
                            9,
                            AnimationType::Walk,
                            true,
                            false,
                        ),
                        (
                            "player/player_hit.png",
                            Vec2::new(96.0, 96.0),
                            (9, 1),
                            8,
                            AnimationType::Hit,
                            false,
                            true,
                        ),
                        (
                            "player/player_death.png",
                            Vec2::new(96.0, 96.0),
                            (52, 1),
                            51,
                            AnimationType::Death,
                            false,
                            false,
                        ),
                    ]
                    .into_iter()
                    .map(|data| data.into())
                    .collect();

                    spawn_animated_children(
                        &asset_server,
                        &mut texture_atlas_layouts,
                        parent,
                        children_params,
                    );
                });
        }
    }
}

pub fn cleanup_game_system(
    commands: &mut Commands,
    characters_query: &Query<Entity, With<Cleanup>>,
) {
    for entity in characters_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
