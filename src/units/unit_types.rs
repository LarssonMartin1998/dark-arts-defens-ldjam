use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Harvester,
    Tank,
    AoeCaster,
    SingleTargetCaster,
}

#[derive(Component)]
pub struct Harvester;
#[derive(Component)]
pub struct Tank;
#[derive(Component)]
pub struct AoeCaster;
#[derive(Component)]
pub struct SingleTargetCaster;

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
                (UnitType::Harvester, UnitConfig { cost: 10 }),
                (UnitType::Tank, UnitConfig { cost: 20 }),
                (UnitType::AoeCaster, UnitConfig { cost: 30 }),
                (UnitType::SingleTargetCaster, UnitConfig { cost: 40 }),
            ]
            .iter()
            .cloned()
            .collect(),
        )
    }
}
