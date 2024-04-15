use crate::animation::{spawn_animated_children, AnimatedChildSpawnParams, AnimationType};
use crate::mana::Mana;
use crate::movement::Movement;
use crate::units::unit_types::UnitBundle;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
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
                ),
                (
                    "player/player_walk.png",
                    Vec2::new(96.0, 96.0),
                    (10, 1),
                    9,
                    AnimationType::Walk,
                    true,
                ),
                (
                    "player/player_hit.png",
                    Vec2::new(96.0, 96.0),
                    (9, 1),
                    8,
                    AnimationType::Hit,
                    false,
                ),
                (
                    "player/player_death.png",
                    Vec2::new(96.0, 96.0),
                    (52, 1),
                    51,
                    AnimationType::Death,
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
