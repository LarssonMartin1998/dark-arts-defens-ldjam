use bevy::prelude::*;

use crate::velocity::Velocity;

#[derive(Clone, Debug)]
pub enum Behavior {
    Idle(IdleBehavior),           // Do nothing
    MoveOrigo(MoveOrigoBehavior), // Special case for enemies with no targets in range, move towards origo instead
    Wander(WanderBehaviorBundle), // Friendly units wander around when waiting for enemies
    Chase(ChaseBehavior),         // Both friendly and enemy units chase their targets
    Flee(FleeBehavior),           // The acolyte tries to flee from enemies
    Attack(AttackBehavior),       // Attack when in range
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior::Wander(WanderBehaviorBundle::default())
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct IdleBehavior {}

#[derive(Component, Clone, Copy, Debug)]
pub struct MoveOrigoBehavior {}

#[derive(Bundle, Clone, Debug)]
pub struct WanderBehaviorBundle {
    pub wander_behavior: WanderBehavior,
    pub wander_timer: WanderBehaviorWanderTimer,
    pub wait_timer: WanderBehaviorWaitTimer,
}

impl Default for WanderBehaviorBundle {
    fn default() -> Self {
        let wander_time = 3.0;
        let wait_time = 1.5;
        WanderBehaviorBundle {
            wander_behavior: WanderBehavior {
                wait_time,
                wander_time,
                random_time_offset: 0.75,
                is_wandering: false,
            },
            wander_timer: WanderBehaviorWanderTimer(Timer::from_seconds(
                wander_time,
                TimerMode::Once,
            )),
            wait_timer: WanderBehaviorWaitTimer(Timer::from_seconds(wait_time, TimerMode::Once)),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct WanderBehaviorWanderTimer(pub Timer);
#[derive(Component, Clone, Debug)]
pub struct WanderBehaviorWaitTimer(pub Timer);
#[derive(Component, Clone, Copy, Debug)]
pub struct WanderBehavior {
    pub wait_time: f32,
    pub wander_time: f32,
    pub random_time_offset: f32,
    pub is_wandering: bool,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct ChaseBehavior {}

#[derive(Component, Clone, Copy, Debug)]
pub struct FleeBehavior {}

#[derive(Component, Clone, Copy, Debug)]
pub struct AttackBehavior {}

#[derive(Component, Default, Clone)]
pub struct CurrentBehavior(pub Behavior);

#[derive(Component, Clone)]
pub struct SupportedBehaviors(pub Vec<(Behavior, u8)>);

impl Default for SupportedBehaviors {
    fn default() -> Self {
        SupportedBehaviors(vec![
            (Behavior::Wander(WanderBehaviorBundle::default()), 5),
            (Behavior::Chase(ChaseBehavior {}), 10),
            (Behavior::Attack(AttackBehavior {}), 15),
        ])
    }
}

#[derive(Bundle, Default, Clone)]
pub struct BehaviorBundle {
    pub current_behavior: CurrentBehavior,
    pub supported_behaviors: SupportedBehaviors,
}

pub fn behavior_state_machine(
    mut query: Query<(&mut CurrentBehavior, &SupportedBehaviors, &Transform)>,
    window_query: Query<&Window>,
) {
    for (mut current_behavior, supported_behaviors, transform) in query.iter_mut() {
        let mut behaviors_that_want_to_be_active = supported_behaviors
            .0
            .iter()
            .filter(|behavior| {
                let behavior_wants_to_be_active = match behavior {
                    (Behavior::Idle(_b), _p) => true,
                    (Behavior::MoveOrigo(_b), _p) => {
                        let window = window_query.single();
                        let distance_to_origo = transform.translation.truncate().length();
                        distance_to_origo > window.height() * 0.2
                    }
                    (Behavior::Wander(_b), _p) => true,
                    (Behavior::Chase(_b), _p) => false,
                    (Behavior::Flee(_b), _p) => false,
                    (Behavior::Attack(_b), _p) => false,
                };

                behavior_wants_to_be_active
            })
            .cloned()
            .collect::<Vec<(Behavior, u8)>>();

        behaviors_that_want_to_be_active.sort_by(|a, b| b.1.cmp(&a.1));
        let highest_prio_behavior = &behaviors_that_want_to_be_active[0].0;

        current_behavior.0 = highest_prio_behavior.clone();
    }
}

pub fn execute_behavior_idle(mut query: Query<(&CurrentBehavior, &mut Velocity)>) {
    for (current_behavior, mut velocity) in query.iter_mut() {
        if let Behavior::Idle(_) = current_behavior.0 {
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn execute_behavior_move_origo(
    mut query: Query<(&CurrentBehavior, &mut Velocity, &Transform)>,
) {
    for (current_behavior, mut velocity, transform) in query.iter_mut() {
        if let Behavior::MoveOrigo(_) = current_behavior.0 {
            let direction = -transform.translation.truncate();
            velocity.0 = direction.normalize_or_zero();
        }
    }
}

pub fn execute_behavior_wander(
    time: Res<Time>,
    mut query: Query<(
        &CurrentBehavior,
        &mut WanderBehavior,
        &mut WanderBehaviorWaitTimer,
        &mut WanderBehaviorWanderTimer,
        &mut Velocity,
    )>,
) {
    for (current_behavior, mut wander_behavior, mut wait_timer, mut wander_timer, mut velocity) in
        query.iter_mut()
    {
        println!("execute_behavior_wander");
        if let Behavior::Wander(_) = current_behavior.0 {
            if wander_behavior.is_wandering {
                if wander_timer.0.tick(time.delta()).just_finished() {
                    wander_behavior.is_wandering = false;
                    wait_timer.0 = Timer::from_seconds(
                        wander_behavior.wait_time
                            + rand::random::<f32>() * wander_behavior.random_time_offset,
                        TimerMode::Once,
                    );

                    velocity.0 = Vec2::ZERO;
                }
            } else if wait_timer.0.tick(time.delta()).just_finished() {
                wander_behavior.is_wandering = true;
                wander_timer.0 = Timer::from_seconds(
                    wander_behavior.wander_time
                        + rand::random::<f32>() * wander_behavior.random_time_offset,
                    TimerMode::Once,
                );

                // randomize the direction of the velocity, and normalize it, then half it,
                // because the units should move slower when is_wandering
                velocity.0 = Vec2::new(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                )
                .normalize()
                    * 0.5;
            }
        }
    }
}

pub fn execute_behavior_chase() {}
pub fn execute_behavior_flee() {}
pub fn execute_behavior_attack() {}
