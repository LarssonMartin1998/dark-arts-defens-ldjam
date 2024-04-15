use bevy::prelude::*;

use crate::ai::behavior;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                behavior::behavior_state_machine,
                behavior::execute_behavior_idle,
                behavior::execute_behavior_move_origo,
                behavior::execute_behavior_wander,
                behavior::execute_behavior_chase,
                behavior::execute_behavior_flee,
                behavior::execute_behavior_attack,
                behavior::execute_behavior_dead,
            ),
        );
    }
}
