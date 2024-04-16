use bevy::prelude::*;
use rand::Rng;

use crate::{
    dark_arts_defense::{GameEvent, RandomSeed},
    units::{
        health::Health,
        team::{CurrentTeam, Team},
    },
    velocity::Velocity,
};

const ATTACK_DISTANCE_MAX: f32 = 96.0;
const ATTACK_DISTANCE_MID: f32 = 72.0;
const ATTACK_DISTANCE_MIN: f32 = 48.0;

#[derive(Clone, Debug)]
pub enum Behavior {
    Idle(IdleBehavior),           // Do nothing
    MoveOrigo(MoveOrigoBehavior), // Special case for enemies with no targets in range, move towards origo instead
    Wander(WanderBehavior),       // Friendly units wander around when waiting for enemies
    Chase(ChaseBehavior),         // Both friendly and enemy units chase their targets
    Flee(FleeBehavior),           // The acolyte tries to flee from enemies
    Attack(AttackBehavior),       // Attack when in range
    Dead(DeadBehavior),           // Dead units do nothing
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior::Wander(WanderBehavior::default())
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct IdleBehavior {}

#[derive(Component, Clone, Copy, Debug)]
pub struct MoveOrigoBehavior {}

#[derive(Component, Clone, Debug)]
pub struct WanderBehavior {
    pub wait_time: f32,
    pub wander_time: f32,
    pub random_time_offset: f32,
    pub is_wandering: bool,
    pub wait_timer: Timer,
    pub wander_timer: Timer,
}

impl Default for WanderBehavior {
    fn default() -> Self {
        let wander_time = 3.0;
        let wait_time = 1.5;
        WanderBehavior {
            wait_time,
            wander_time,
            random_time_offset: 0.75,
            is_wandering: false,
            wait_timer: Timer::from_seconds(wait_time, TimerMode::Once),
            wander_timer: Timer::from_seconds(wander_time, TimerMode::Once),
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct ChaseBehavior {}

#[derive(Component, Clone, Copy, Debug)]
pub struct FleeBehavior {}

#[derive(Component, Clone, Debug)]
pub struct AttackBehavior {
    pub cooldown: f32,
    pub random_cooldown_offset: f32,
    pub random_attack_offset: u8,
    pub damage: u8,
    pub is_attacking: bool,
    pub timer: Timer,
}

impl Default for AttackBehavior {
    fn default() -> Self {
        let attack_cooldown = 4.0;
        AttackBehavior {
            cooldown: attack_cooldown,
            random_cooldown_offset: 0.5,
            random_attack_offset: 5,
            damage: 10,
            is_attacking: false,
            timer: Timer::from_seconds(attack_cooldown, TimerMode::Once),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct DeadBehavior;

#[derive(Component, Default, Clone)]
pub struct CurrentBehavior(pub Behavior);

#[derive(Component, Clone)]
pub struct SupportedBehaviors(pub Vec<(Behavior, u8)>);

impl Default for SupportedBehaviors {
    fn default() -> Self {
        SupportedBehaviors(vec![
            (Behavior::Wander(WanderBehavior::default()), 5),
            (Behavior::Chase(ChaseBehavior {}), 10),
            (Behavior::Attack(AttackBehavior::default()), 15),
            (Behavior::Dead(DeadBehavior {}), 20),
        ])
    }
}

#[derive(Bundle, Default, Clone)]
pub struct BehaviorBundle {
    pub current_behavior: CurrentBehavior,
    pub supported_behaviors: SupportedBehaviors,
}

fn get_flee_distance(window: &Window) -> f32 {
    window.width() * 0.15
}

fn get_chase_distance(window: &Window) -> f32 {
    window.width() * 0.4
}

fn is_other_valid_target(
    team: &CurrentTeam,
    other_health: &Health,
    other_team: &CurrentTeam,
    transform: &Transform,
    other_transform: &Transform,
    distance: f32,
) -> bool {
    if team.is_friendly(other_team) {
        return false;
    }

    if other_health.is_dead() {
        return false;
    }

    let distance_to_other =
        transform.translation.truncate() - other_transform.translation.truncate();
    distance_to_other.length() < distance
}

pub fn behavior_state_machine(
    mut query: Query<(
        &mut CurrentBehavior,
        &SupportedBehaviors,
        &Transform,
        &CurrentTeam,
        &Health,
    )>,
    others_query: Query<(&Transform, &CurrentTeam, &Health)>,
    window_query: Query<&Window>,
) {
    for (mut current_behavior, supported_behaviors, transform, team, health) in query.iter_mut() {
        let window = &window_query.single();
        let mut behaviors_that_want_to_be_active = supported_behaviors
            .0
            .iter()
            .filter(|behavior| {
                let behavior_wants_to_be_active =
                    match behavior {
                        (Behavior::Idle(_b), _p) => true,
                        (Behavior::MoveOrigo(_b), _p) => {
                            let window = window_query.single();
                            let distance_to_origo = transform.translation.truncate().length();
                            distance_to_origo > window.height() * 0.3
                        }
                        (Behavior::Wander(_b), _p) => true,
                        (Behavior::Chase(_b), _p) => others_query.iter().any(
                            |(other_transform, other_team, other_health)| {
                                is_other_valid_target(
                                    team,
                                    other_health,
                                    other_team,
                                    transform,
                                    other_transform,
                                    get_chase_distance(window),
                                )
                            },
                        ),
                        (Behavior::Flee(_b), _p) => others_query.iter().any(
                            |(other_transform, other_team, other_health)| {
                                is_other_valid_target(
                                    team,
                                    other_health,
                                    other_team,
                                    transform,
                                    other_transform,
                                    get_flee_distance(window),
                                )
                            },
                        ),
                        (Behavior::Attack(_b), _p) => others_query.iter().any(
                            |(other_transform, other_team, other_health)| {
                                is_other_valid_target(
                                    team,
                                    other_health,
                                    other_team,
                                    transform,
                                    other_transform,
                                    ATTACK_DISTANCE_MAX,
                                )
                            },
                        ),
                        (Behavior::Dead(_b), _p) => health.is_dead(),
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

pub fn execute_behavior_idle(mut query: Query<(&CurrentBehavior, &IdleBehavior, &mut Velocity)>) {
    for (current_behavior, _, mut velocity) in query.iter_mut() {
        if let Behavior::Idle(_) = current_behavior.0 {
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn execute_behavior_move_origo(
    mut query: Query<(
        &CurrentBehavior,
        &MoveOrigoBehavior,
        &mut Velocity,
        &Transform,
    )>,
) {
    for (current_behavior, _, mut velocity, transform) in query.iter_mut() {
        if let Behavior::MoveOrigo(_) = current_behavior.0 {
            let direction = -transform.translation.truncate();
            velocity.0 = direction.normalize_or_zero();
        }
    }
}

pub fn execute_behavior_wander(
    time: Res<Time>,
    mut query: Query<(&CurrentBehavior, &mut WanderBehavior, &mut Velocity)>,
) {
    for (current_behavior, mut wander_behavior, mut velocity) in query.iter_mut() {
        if let Behavior::Wander(_) = current_behavior.0 {
            if wander_behavior.is_wandering {
                if wander_behavior
                    .wander_timer
                    .tick(time.delta())
                    .just_finished()
                {
                    wander_behavior.is_wandering = false;
                    wander_behavior.wait_timer = Timer::from_seconds(
                        wander_behavior.wait_time
                            + rand::random::<f32>() * wander_behavior.random_time_offset,
                        TimerMode::Once,
                    );

                    velocity.0 = Vec2::ZERO;
                }
            } else if wander_behavior
                .wait_timer
                .tick(time.delta())
                .just_finished()
            {
                wander_behavior.is_wandering = true;
                wander_behavior.wander_timer = Timer::from_seconds(
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

pub fn execute_behavior_chase(
    mut query: Query<(
        &CurrentBehavior,
        &ChaseBehavior,
        &Transform,
        &CurrentTeam,
        &mut Velocity,
    )>,
    window_query: Query<&Window>,
    others_query: Query<(&Transform, &CurrentTeam, &Health)>,
) {
    query
        .iter_mut()
        .for_each(|(current_behavior, _, transform, team, mut velocity)| {
            if let Behavior::Chase(_) = current_behavior.0 {
                let window = window_query.single();
                let mut enemies_within_range = others_query
                    .iter()
                    .filter(|(other_transform, other_team, other_health)| {
                        is_other_valid_target(
                            team,
                            other_health,
                            other_team,
                            transform,
                            other_transform,
                            get_chase_distance(window),
                        )
                    })
                    .collect::<Vec<(&Transform, &CurrentTeam, &Health)>>();

                enemies_within_range.sort_by(|a, b| {
                    let distance_to_a =
                        transform.translation.truncate() - a.0.translation.truncate();
                    let distance_to_b =
                        transform.translation.truncate() - b.0.translation.truncate();
                    distance_to_a
                        .length()
                        .partial_cmp(&distance_to_b.length())
                        .unwrap()
                });

                if let Some((enemy_transform, _t, _h)) = enemies_within_range.first() {
                    let direction =
                        enemy_transform.translation.truncate() - transform.translation.truncate();
                    velocity.0 = direction.normalize_or_zero();
                }
            }
        });
}

pub fn execute_behavior_flee(
    window_query: Query<&Window>,
    mut query: Query<(
        &CurrentBehavior,
        &FleeBehavior,
        &Transform,
        &CurrentTeam,
        &mut Velocity,
    )>,
    others_query: Query<(&Transform, &CurrentTeam, &Health)>,
) {
    let window = window_query.single();
    query
        .iter_mut()
        .for_each(|(current_behavior, _, transform, team, mut velocity)| {
            if let Behavior::Flee(_) = current_behavior.0 {
                let enemies_within_range = others_query
                    .iter()
                    .filter(|(other_transform, other_team, other_health)| {
                        is_other_valid_target(
                            team,
                            other_health,
                            other_team,
                            transform,
                            other_transform,
                            get_flee_distance(window),
                        )
                    })
                    .collect::<Vec<(&Transform, &CurrentTeam, &Health)>>();

                let center_of_mass = enemies_within_range.iter().fold(
                    (Vec2::ZERO, 0.0),
                    |mut acc, (other_transform, _, _)| {
                        let distance_to_other = (transform.translation.truncate()
                            - other_transform.translation.truncate())
                        .length();
                        let weight = 1.0 / distance_to_other;
                        acc.0 += other_transform.translation.truncate() * weight;
                        acc.1 += weight;
                        acc
                    },
                );

                let flee_from = center_of_mass.0 / center_of_mass.1;
                velocity.0 = (transform.translation.truncate() - flee_from).normalize_or_zero();
            };
        });
}

pub fn execute_behavior_attack(
    time: Res<Time>,
    mut rng: ResMut<RandomSeed>,
    mut query: Query<(
        &CurrentBehavior,
        &mut AttackBehavior,
        &Transform,
        &CurrentTeam,
        &mut Velocity,
    )>,
    mut others_query: Query<(&Transform, &CurrentTeam, &mut Health)>,
    mut event_writer: EventWriter<GameEvent>,
) {
    query.iter_mut().for_each(
        |(current_behavior, mut attack_behavior, transform, team, mut velocity)| {
            if let Behavior::Attack(_) = current_behavior.0 {
                let mut enemies_within_range = others_query
                    .iter_mut()
                    .filter(|(other_transform, other_team, other_health)| {
                        is_other_valid_target(
                            team,
                            other_health,
                            other_team,
                            transform,
                            other_transform,
                            ATTACK_DISTANCE_MAX,
                        )
                    })
                    .collect::<Vec<(&Transform, &CurrentTeam, Mut<Health>)>>();

                enemies_within_range.sort_by(|a, b| {
                    let distance_to_a =
                        transform.translation.truncate() - a.0.translation.truncate();
                    let distance_to_b =
                        transform.translation.truncate() - b.0.translation.truncate();
                    distance_to_a
                        .length()
                        .partial_cmp(&distance_to_b.length())
                        .unwrap()
                });

                if let Some((enemy_transform, enemy_team, enemy_health)) =
                    enemies_within_range.first_mut()
                {
                    let direction =
                        enemy_transform.translation.truncate() - transform.translation.truncate();

                    velocity.0 = if direction.length() > ATTACK_DISTANCE_MID {
                        direction.normalize_or_zero()
                    } else if direction.length() > ATTACK_DISTANCE_MIN {
                        velocity.0
                    } else {
                        Vec2::ZERO
                    };

                    if attack_behavior.timer.tick(time.delta()).just_finished() {
                        let final_damage = std::cmp::min(
                            rng.0.gen_range(
                                attack_behavior.damage
                                    ..=attack_behavior.damage
                                        + attack_behavior.random_attack_offset,
                            ),
                            enemy_health.0,
                        );
                        enemy_health.0 -= final_damage;
                        if enemy_health.is_dead() && enemy_team.0 == Team::Good {
                            event_writer.send(GameEvent::IncreaseScore);
                        }

                        let new_cooldown = attack_behavior.cooldown
                            + rand::random::<f32>() * attack_behavior.random_cooldown_offset;
                        attack_behavior.timer = Timer::from_seconds(new_cooldown, TimerMode::Once);
                        attack_behavior.is_attacking = true;
                    }
                }
            }
        },
    );
}

pub fn execute_behavior_dead(mut query: Query<(&CurrentBehavior, &DeadBehavior, &mut Velocity)>) {
    for (current_behavior, _, mut velocity) in query.iter_mut() {
        if let Behavior::Dead(_) = current_behavior.0 {
            velocity.0 = Vec2::ZERO;
        }
    }
}
