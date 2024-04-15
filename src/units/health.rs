use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub u8);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
}
