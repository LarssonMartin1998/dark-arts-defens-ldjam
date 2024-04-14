use bevy::prelude::*;

use crate::ai::behavior;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, behavior::execute_current_behavior);
    }
}
