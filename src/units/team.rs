use bevy::prelude::*;

#[derive(Eq, PartialEq, Default)]
pub enum Team {
    #[default]
    Evil, // In this game, the player is evil
    Good,
}

#[derive(Component, Default)]
pub struct CurrentTeam(pub Team);

impl CurrentTeam {
    pub fn is_friendly(&self, other: &CurrentTeam) -> bool {
        self.0 == other.0
    }
}
