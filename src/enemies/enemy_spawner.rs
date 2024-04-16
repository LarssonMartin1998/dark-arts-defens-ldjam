use bevy::prelude::*;
use bevy::window::Window;

use crate::enemies::plugin::SpawnTimer;
use crate::units::team::Team;
use crate::units::unit_types::{spawn_unit, Knight};

enum EnemyDirection {
    Top,
    Right,
    Bottom,
    Left,
}

impl EnemyDirection {
    fn new() -> Self {
        match rand::random::<u8>() % 4 {
            0 => Self::Top,
            1 => Self::Right,
            2 => Self::Bottom,
            3 => Self::Left,
            _ => panic!("Invalid random direction"),
        }
    }
}

const ENEMY_SPAWN_OFFSET: f32 = 256.0;

#[derive(Component)]
pub struct EnemySpawner;

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window_query: Query<&Window>,
    enemy_spawner_query: Query<&EnemySpawner>,
) {
    if enemy_spawner_query.iter().count() == 0 {
        return;
    }

    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let window = window_query.single();
    let play_area = Vec2::new(window.width(), window.height());

    // Randomize a direction for the enemy to spawn from, either top, right, bottom, or left
    // The enemies will have a random offset from the edge of the screen of the chosen direction.
    // The offset will be within the range of 0 to ENEMY_SPAWN_OFFSET
    // The enemy will spawn at a random position along the chosen edge, which will be from 0, and
    // and matching the play_area dimension perpendicular to the chosen edge.
    let random_direction = EnemyDirection::new();
    let random_offset = rand::random::<f32>() * ENEMY_SPAWN_OFFSET;
    let spawn_position = match random_direction {
        EnemyDirection::Top => Vec2::new(
            rand::random::<f32>() * play_area.x - play_area.x * 0.5,
            play_area.y * 0.5 + random_offset,
        ),
        EnemyDirection::Right => Vec2::new(
            play_area.x * 0.5 + random_offset,
            rand::random::<f32>() * play_area.y - play_area.y * 0.5,
        ),
        EnemyDirection::Bottom => Vec2::new(
            rand::random::<f32>() * play_area.x - play_area.x * 0.5,
            -play_area.y * 0.5 - random_offset,
        ),
        EnemyDirection::Left => Vec2::new(
            -play_area.x * 0.5 - random_offset,
            rand::random::<f32>() * play_area.y - play_area.y * 0.5,
        ),
    };

    spawn_unit(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        Knight,
        Team::Good,
        spawn_position,
    );
}
