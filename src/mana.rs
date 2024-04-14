use bevy::prelude::*;

#[derive(Component)]
pub struct Mana {
    pub current_mana: u8,
    pub max_mana: u8,
}
