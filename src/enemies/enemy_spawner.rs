use bevy::prelude::*;
use bevy::window::Window;

use crate::ai::behavior::{
    Behavior, BehaviorBundle, CurrentBehavior, MoveOrigoBehavior, SupportedBehaviors,
    WanderBehaviorBundle,
};
use crate::animation::spawn_animated_children;
use crate::enemies::plugin::SpawnTimer;
use crate::units::unit_types::{Cat, UnitChildrenSpawnParamsFactory};

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

const ENEMY_SPAWN_OFFSET: f32 = -256.0;

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window_query: Query<&Window>,
) {
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

    let cat = Cat;
    let mut bundle = cat.create_bundle();
    bundle.transform.translation = Vec3::new(spawn_position.x, spawn_position.y, 0.0);
    let start_behavior = Behavior::MoveOrigo(MoveOrigoBehavior {});
    let supported_behaviors = SupportedBehaviors(vec![
        (start_behavior.clone(), 5),
        (Behavior::Wander(WanderBehaviorBundle::default()), 3),
    ]);
    let mut entity = commands.spawn((
        bundle,
        BehaviorBundle {
            current_behavior: CurrentBehavior(start_behavior),
            supported_behaviors: supported_behaviors.clone(),
        },
    ));

    supported_behaviors.0.iter().for_each(|behavior| {
        match behavior {
            (Behavior::Idle(behavior), _) => {
                entity.insert(*behavior);
            }
            (Behavior::MoveOrigo(behavior), _) => {
                entity.insert(*behavior);
            }
            (Behavior::Wander(behavior), _) => {
                entity.insert(behavior.clone());
            }
            (Behavior::Chase(behavior), _) => {
                entity.insert(*behavior);
            }
            (Behavior::Flee(behavior), _) => {
                entity.insert(*behavior);
            }
            (Behavior::Attack(behavior), _) => {
                entity.insert(*behavior);
            }
        };
    });

    entity.with_children(|parent| {
        spawn_animated_children(
            &asset_server,
            &mut texture_atlas_layouts,
            parent,
            cat.create_children_spawn_params(),
        );
    });
}
