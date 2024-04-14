use bevy::prelude::*;

use crate::velocity::Velocity;

// The u8 is used to configure priority of behaviors that meet their conditions
// The higher the number, the higher the priority
pub enum Behavior {
    Idle(u8),      // Do nothing
    MoveOrigo(u8), // Special case for enemies with no targets in range, move towards origo instead
    Wander(u8),    // Friendly units wander around when waiting for enemies
    Chase(u8),     // Both friendly and enemy units chase their targets
    Flee(u8),      // The acolyte tries to flee from enemies
    Attack(u8),    // Attack when in range
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior::Idle(0)
    }
}

#[derive(Component, Default)]
pub struct CurrentBehavior(pub Behavior);

#[derive(Component)]
pub struct SupportedBehaviors(pub Vec<Behavior>);

impl Default for SupportedBehaviors {
    fn default() -> Self {
        SupportedBehaviors(vec![
            Behavior::Wander(4),
            Behavior::Chase(7),
            Behavior::Attack(10),
        ])
    }
}

#[derive(Bundle, Default)]
pub struct BehaviorBundle {
    pub current_behavior: CurrentBehavior,
    pub supported_behaviors: SupportedBehaviors,
}

pub fn behavior_state_machine() {}

pub fn execute_current_behavior(mut query: Query<(&CurrentBehavior, &mut Velocity)>) {
    for (current_behavior, mut velocity) in query.iter_mut() {
        match current_behavior.0 {
            Behavior::Idle(_) => {}
            Behavior::MoveOrigo(_) => {}
            Behavior::Wander(_) => {
                // Randomize a direction for the unit to move in
                let direction = Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
                velocity.0 = direction.normalize();
            }
            Behavior::Chase(_) => {}
            Behavior::Flee(_) => {}
            Behavior::Attack(_) => {}
        }
    }
}
