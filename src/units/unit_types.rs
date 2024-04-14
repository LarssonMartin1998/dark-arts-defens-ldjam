use crate::ai::behavior::{
    AttackBehaviorBundle, Behavior, BehaviorBundle, ChaseBehavior, CurrentBehavior,
    MoveOrigoBehavior, SupportedBehaviors, WanderBehaviorBundle,
};
use crate::animation::{spawn_animated_children, CurrentAnimation};
use crate::animation::{AnimatedChildSpawnParams, AnimationType};
use crate::movement::Movement;
use crate::units::{health::Health, team::CurrentTeam};
use crate::velocity::Velocity;
use bevy::prelude::*;
use std::collections::HashMap;

use super::team::Team;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Acolyte,
    Warrior,
    Cat,

    Knight,
}

#[derive(Bundle, Default)]
pub struct UnitBundle {
    pub movement: Movement,
    pub velocity: Velocity,
    pub current_animation: CurrentAnimation,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
    pub health: Health,
    pub team: CurrentTeam,
}

// Create a trait that will be used to define the components of the units
pub trait UnitChildrenSpawnParamsFactory {
    fn create_unit_bundle(&self) -> UnitBundle;
    fn create_behavior_bundle(&self) -> BehaviorBundle;
    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams>;
}

#[derive(Component)]
pub struct Acolyte;
impl UnitChildrenSpawnParamsFactory for Acolyte {
    fn create_unit_bundle(&self) -> UnitBundle {
        UnitBundle {
            movement: Movement { speed: 75.0 },
            transform: Transform::from_scale(Vec3::splat(0.8)),
            ..default()
        }
    }

    fn create_behavior_bundle(&self) -> BehaviorBundle {
        BehaviorBundle::default()
    }

    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams> {
        [
            (
                "acolyte/acolyte_idle.png",
                Vec2::new(80.0, 80.0),
                (3, 4),
                9,
                AnimationType::Idle,
            ),
            (
                "acolyte/acolyte_idle.png",
                Vec2::new(80.0, 80.0),
                (3, 4),
                9,
                AnimationType::Walk,
            ),
            (
                "acolyte/acolyte_death.png",
                Vec2::new(80.0, 80.0),
                (3, 4),
                9,
                AnimationType::Death,
            ),
        ]
        .into_iter()
        .map(|data| data.into())
        .collect()
    }
}

#[derive(Component)]
pub struct Warrior;
impl UnitChildrenSpawnParamsFactory for Warrior {
    fn create_unit_bundle(&self) -> UnitBundle {
        UnitBundle {
            movement: Movement { speed: 200.0 },
            transform: Transform::from_scale(Vec3::splat(1.8)),
            ..default()
        }
    }

    fn create_behavior_bundle(&self) -> BehaviorBundle {
        BehaviorBundle::default()
    }

    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams> {
        [
            (
                "warrior/warrior_idle.png",
                Vec2::new(96.0, 96.0),
                (21, 1),
                20,
                AnimationType::Idle,
            ),
            (
                "warrior/warrior_walk.png",
                Vec2::new(96.0, 96.0),
                (11, 1),
                10,
                AnimationType::Walk,
            ),
            (
                "warrior/warrior_hit.png",
                Vec2::new(96.0, 96.0),
                (9, 1),
                8,
                AnimationType::Hit,
            ),
            (
                "warrior/warrior_death.png",
                Vec2::new(96.0, 96.0),
                (36, 1),
                35,
                AnimationType::Death,
            ),
        ]
        .into_iter()
        .map(|data| data.into())
        .collect()
    }
}

#[derive(Component)]
pub struct Cat;
impl UnitChildrenSpawnParamsFactory for Cat {
    fn create_unit_bundle(&self) -> UnitBundle {
        UnitBundle {
            movement: Movement { speed: 250.0 },
            transform: Transform::from_scale(Vec3::splat(1.4)),
            ..default()
        }
    }

    fn create_behavior_bundle(&self) -> BehaviorBundle {
        BehaviorBundle::default()
    }

    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams> {
        [
            (
                "cat/cat_idle.png",
                Vec2::new(96.0, 96.0),
                (10, 1),
                9,
                AnimationType::Idle,
            ),
            (
                "cat/cat_walk.png",
                Vec2::new(96.0, 96.0),
                (8, 1),
                7,
                AnimationType::Walk,
            ),
            (
                "cat/cat_hit.png",
                Vec2::new(96.0, 96.0),
                (9, 1),
                8,
                AnimationType::Hit,
            ),
            (
                "cat/cat_death.png",
                Vec2::new(96.0, 96.0),
                (18, 1),
                17,
                AnimationType::Death,
            ),
        ]
        .into_iter()
        .map(|data| data.into())
        .collect()
    }
}

#[derive(Component)]
pub struct Knight;
impl UnitChildrenSpawnParamsFactory for Knight {
    fn create_unit_bundle(&self) -> UnitBundle {
        UnitBundle {
            movement: Movement { speed: 250.0 },
            transform: Transform::from_scale(Vec3::splat(1.5)),
            ..default()
        }
    }

    fn create_behavior_bundle(&self) -> BehaviorBundle {
        BehaviorBundle {
            supported_behaviors: SupportedBehaviors(vec![
                (Behavior::Wander(WanderBehaviorBundle::default()), 3),
                (Behavior::MoveOrigo(MoveOrigoBehavior {}), 5),
                (Behavior::Chase(ChaseBehavior {}), 10),
                (Behavior::Attack(AttackBehaviorBundle::default()), 15),
            ]),
            current_behavior: CurrentBehavior(Behavior::MoveOrigo(MoveOrigoBehavior {})),
        }
    }

    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams> {
        [
            (
                "enemy/enemy_idle.png",
                Vec2::new(64.0, 64.0),
                (12, 1),
                11,
                AnimationType::Idle,
            ),
            (
                "enemy/enemy_move.png",
                Vec2::new(96.0, 64.0),
                (8, 1),
                7,
                AnimationType::Walk,
            ),
            (
                "enemy/enemy_death.png",
                Vec2::new(96.0, 64.0),
                (15, 1),
                14,
                AnimationType::Death,
            ),
            (
                "enemy/enemy_attack.png",
                Vec2::new(144.0, 64.0),
                (22, 1),
                21,
                AnimationType::Attack,
            ),
        ]
        .into_iter()
        .map(|data| data.into())
        .collect()
    }
}
#[derive(Resource)]
pub struct UnitResource(HashMap<UnitType, UnitConfig>);

impl UnitResource {
    pub fn get(&self, unit_type: UnitType) -> &UnitConfig {
        &self.0[&unit_type]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitConfig {
    pub cost: u32,
}

impl Default for UnitResource {
    fn default() -> Self {
        Self(
            [
                (UnitType::Acolyte, UnitConfig { cost: 45 }),
                (UnitType::Warrior, UnitConfig { cost: 30 }),
                (UnitType::Cat, UnitConfig { cost: 20 }),
            ]
            .iter()
            .cloned()
            .collect(),
        )
    }
}

pub fn spawn_unit(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    unit_component: impl UnitChildrenSpawnParamsFactory,
    team: Team,
    spawn_position: Vec2,
) {
    let mut unit_bundle = unit_component.create_unit_bundle();
    unit_bundle.team = CurrentTeam(team);
    unit_bundle.transform.translation = Vec3::new(spawn_position.x, spawn_position.y, 0.0);

    let behavior_bundle = unit_component.create_behavior_bundle();
    let mut entity = commands.spawn((unit_bundle, behavior_bundle.clone()));

    behavior_bundle
        .supported_behaviors
        .0
        .iter()
        .for_each(|behavior| {
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
                    entity.insert(behavior.clone());
                }
            };
        });

    entity.with_children(|parent| {
        spawn_animated_children(
            asset_server,
            texture_atlas_layouts,
            parent,
            unit_component.create_children_spawn_params(),
        );
    });
}
