use crate::{ai::behavior::AttackBehavior, units::health::Health, velocity::Velocity};
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AnimationType {
    #[default]
    Idle,
    Walk,
    Hit,
    Death,
    Attack,
}

#[derive(Component, Clone, Default)]
pub struct Animation {
    pub animation_type: AnimationType,
    pub last_atlas_index: usize,
    pub is_looping: bool,
    pub frame_timer: Timer,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
pub struct CurrentAnimation {
    pub animation_type: AnimationType,
}

#[derive(Bundle, Clone, Default)]
pub struct AnimationBundle {
    /// Specifies the rendering properties of the sprite, such as color tint and flip.
    pub sprite: Sprite,
    /// The local transform of the sprite, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the sprite. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// The sprite sheet base texture
    pub texture: Handle<Image>,
    /// The sprite sheet texture atlas, allowing to draw a custom section of `texture`.
    pub atlas: TextureAtlas,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,

    pub animation: Animation,
}

pub struct AnimatedChildSpawnParams {
    pub texture_path: String,
    pub tile_size: Vec2,
    pub grid: (usize, usize),
    pub last_atlas_index: usize,
    pub animation_type: AnimationType,
    pub is_looping: bool,
    pub is_locked: bool,
}

impl From<(&str, Vec2, (usize, usize), usize, AnimationType, bool, bool)>
    for AnimatedChildSpawnParams
{
    fn from(item: (&str, Vec2, (usize, usize), usize, AnimationType, bool, bool)) -> Self {
        Self {
            texture_path: item.0.to_owned(),
            tile_size: item.1,
            grid: item.2,
            last_atlas_index: item.3,
            animation_type: item.4,
            is_looping: item.5,
            is_locked: item.6,
        }
    }
}

pub fn spawn_animated_children(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    parent: &mut ChildBuilder,
    children_params: Vec<AnimatedChildSpawnParams>,
) {
    children_params.into_iter().for_each(|child_param| {
        let layout = TextureAtlasLayout::from_grid(
            child_param.tile_size,
            child_param.grid.0,
            child_param.grid.1,
            None,
            None,
        );

        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        parent.spawn(AnimationBundle {
            texture: asset_server.load(child_param.texture_path),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::default(),
            animation: Animation {
                animation_type: child_param.animation_type,
                last_atlas_index: child_param.last_atlas_index,
                is_looping: child_param.is_looping,
                frame_timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
            ..Default::default()
        });
    });
}

// Don't we just love hacky game jam code?
fn get_animation_type(
    health: &Health,
    velocity: &Velocity,
    children: &Children,
    attack_behavior: Option<&mut AttackBehavior>,
    child_query: &mut Query<(&mut Sprite, &mut Animation, &mut TextureAtlas)>,
) -> AnimationType {
    let run_attack = match attack_behavior {
        Some(attack_behavior) => attack_behavior.is_attacking,
        None => false,
    };

    if health.is_dead() {
        AnimationType::Death
    } else if run_attack {
        AnimationType::Attack
    } else if velocity.0.length() > 0.0 {
        for child in children.iter() {
            if let Ok((mut sprite, _, _)) = child_query.get_mut(*child) {
                if velocity.0.x != 0.0 {
                    sprite.flip_x = velocity.0.x < 0.0;
                }
            }
        }

        AnimationType::Walk
    } else {
        AnimationType::Idle
    }
}

fn update_current_animation(
    current_animation: &mut CurrentAnimation,
    animation_type: AnimationType,
    children: &Children,
    child_query: &mut Query<(&mut Sprite, &mut Animation, &mut TextureAtlas)>,
) {
    if current_animation.animation_type == animation_type {
        return;
    }

    current_animation.animation_type = animation_type;
    for child in children.iter() {
        if let Ok((_, mut animation, mut atlas)) = child_query.get_mut(*child) {
            if animation.animation_type == current_animation.animation_type {
                atlas.index = 0;
                animation.frame_timer.reset();
            }
        }
    }
}

pub fn animation_state_machine(
    mut query: Query<
        (&mut CurrentAnimation, &Health, &Velocity, &Children),
        Without<AttackBehavior>,
    >,
    mut query_with_attack: Query<(
        &mut CurrentAnimation,
        &Health,
        &Velocity,
        &mut AttackBehavior,
        &Children,
    )>,
    mut child_query: Query<(&mut Sprite, &mut Animation, &mut TextureAtlas)>,
) {
    for (mut current_animation, health, velocity, children) in query.iter_mut() {
        update_current_animation(
            &mut current_animation,
            get_animation_type(health, velocity, children, None, &mut child_query),
            children,
            &mut child_query,
        );
    }
    for (mut current_animation, health, velocity, mut attack_behavior, children) in
        query_with_attack.iter_mut()
    {
        update_current_animation(
            &mut current_animation,
            get_animation_type(
                health,
                velocity,
                children,
                Some(&mut attack_behavior),
                &mut child_query,
            ),
            children,
            &mut child_query,
        );
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query_with: Query<(&CurrentAnimation, &Children, &mut AttackBehavior)>,
    query_without: Query<(&CurrentAnimation, &Children), Without<AttackBehavior>>,
    mut child_query: Query<(&mut Animation, &mut TextureAtlas)>,
) {
    let combined_children: Vec<(&CurrentAnimation, &Children, Option<Mut<AttackBehavior>>)> =
        query_with
            .iter_mut()
            .map(|(current_anim, children, attack_behavior)| {
                (current_anim, children, Some(attack_behavior))
            }) // Retain Mut<AttackBehavior>
            .chain(
                query_without
                    .iter()
                    .map(|(current_anim, children)| (current_anim, children, None)),
            ) // Append children without AttackBehavior
            .collect();

    for (current_anim, children, mut attack_behavior) in combined_children {
        for child in children.iter() {
            if let Ok((mut animation, mut atlas)) = child_query.get_mut(*child) {
                if current_anim.animation_type != animation.animation_type {
                    continue;
                }

                if animation.frame_timer.tick(time.delta()).just_finished() {
                    atlas.index = if atlas.index == animation.last_atlas_index {
                        if let Some(ref mut attack_behavior) = attack_behavior {
                            if attack_behavior.is_attacking {
                                attack_behavior.is_attacking = false;
                            }
                        }

                        if animation.is_looping {
                            animation.frame_timer.reset();
                            0
                        } else {
                            animation.last_atlas_index
                        }
                    } else {
                        animation.frame_timer.reset();
                        atlas.index + 1
                    };
                }
            }
        }
    }
}

pub fn update_animation_visibility(
    query: Query<(&Children, &CurrentAnimation)>,
    mut animation_query: Query<(Entity, &mut Visibility, &Animation)>,
) {
    for (children, current_animation) in query.iter() {
        for &child in children.iter() {
            if let Ok((_, mut visibility, animation)) = animation_query.get_mut(child) {
                *visibility = if current_animation.animation_type == animation.animation_type {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
