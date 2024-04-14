use bevy::prelude::*;

#[derive(Eq, PartialEq, Default, Clone)]
pub enum Team {
    #[default]
    Evil, // In this game, the player is evil
    Good,
}

#[derive(Component, Default, Clone)]
pub struct CurrentTeam(pub Team);

impl CurrentTeam {
    pub fn is_friendly(&self, other: &CurrentTeam) -> bool {
        self.0 == other.0
    }
}
