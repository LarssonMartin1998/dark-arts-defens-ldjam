use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.0)
    }
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.0 <= 0.0
    }
}
