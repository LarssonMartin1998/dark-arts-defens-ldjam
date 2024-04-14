use crate::animation::CurrentAnimation;
use crate::animation::{AnimatedChildSpawnParams, AnimationType};
use crate::movement::Movement;
use crate::velocity::Velocity;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Acolyte,
    Warrior,
    Cat,
}

#[derive(Bundle, Default)]
pub struct UnitBundle {
    pub movement: Movement,
    pub velocity: Velocity,
    pub current_animation: CurrentAnimation,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
}

// Create a trait that will be used to define the components of the units
pub trait UnitChildrenSpawnParamsFactory {
    fn create_bundle(&self) -> UnitBundle;
    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams>;
}

#[derive(Component)]
pub struct Acolyte;
impl UnitChildrenSpawnParamsFactory for Acolyte {
    fn create_bundle(&self) -> UnitBundle {
        UnitBundle {
            movement: Movement { speed: 75.0 },
            ..default()
        }
    }

    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams> {
        [
            ("acolyte/acolyte_idle.png", (50, 1), 49, AnimationType::Idle),
            (
                "acolyte/acolyte_death.png",
                (52, 1),
                51,
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
    fn create_bundle(&self) -> UnitBundle {
        UnitBundle {
            movement: Movement { speed: 200.0 },
            ..default()
        }
    }

    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams> {
        [
            ("warrior/warrior_idle.png", (50, 1), 49, AnimationType::Idle),
            ("warrior/warrior_walk.png", (10, 1), 9, AnimationType::Walk),
            ("warrior/warrior_hit.png", (9, 1), 8, AnimationType::Hit),
            (
                "warrior/warrior_death.png",
                (52, 1),
                51,
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
    fn create_bundle(&self) -> UnitBundle {
        UnitBundle {
            movement: Movement { speed: 250.0 },
            ..default()
        }
    }

    fn create_children_spawn_params(&self) -> Vec<AnimatedChildSpawnParams> {
        [
            ("cat/cat_idle.png", (50, 1), 49, AnimationType::Idle),
            ("cat/cat_walk.png", (10, 1), 9, AnimationType::Walk),
            ("cat/cat_hit.png", (9, 1), 8, AnimationType::Hit),
            ("cat/cat_death.png", (52, 1), 51, AnimationType::Death),
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
