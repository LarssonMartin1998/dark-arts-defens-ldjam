use crate::{units::health::Health, velocity::Velocity};
use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AnimationType {
    #[default]
    Idle,
    Walk,
    Hit,
    Death,
    Attack,
}

#[derive(Component, Clone, Default)]
pub struct LoopAnimation(pub bool);

#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut, Default, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
pub struct CurrentAnimation(pub AnimationType);

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

    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub animation_type: AnimationType,
    pub loop_animation: LoopAnimation,
}

pub struct AnimatedChildSpawnParams {
    pub texture_path: String,
    pub tile_size: Vec2,
    pub grid: (usize, usize),
    pub last_index: usize,
    pub animation_type: AnimationType,
    pub is_looping: bool,
}

impl From<(&str, Vec2, (usize, usize), usize, AnimationType, bool)> for AnimatedChildSpawnParams {
    fn from(item: (&str, Vec2, (usize, usize), usize, AnimationType, bool)) -> Self {
        Self {
            texture_path: item.0.to_owned(),
            tile_size: item.1,
            grid: item.2,
            last_index: item.3,
            animation_type: item.4,
            is_looping: item.5,
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
        let animation_indices = AnimationIndices {
            first: 0,
            last: child_param.last_index,
        };

        parent.spawn(AnimationBundle {
            texture: asset_server.load(child_param.texture_path),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::default(),
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Once)),
            animation_type: child_param.animation_type,
            loop_animation: LoopAnimation(child_param.is_looping),
            ..Default::default()
        });
    });
}

pub fn animation_state_machine(
    mut query: Query<(&mut CurrentAnimation, &Health, &Velocity, &Children)>,
    mut child_query: Query<(&mut Sprite, &mut AnimationTimer)>,
) {
    for (mut current_animation, health, velocity, children) in query.iter_mut() {
        let new_animation = if health.is_dead() {
            AnimationType::Death
        } else if velocity.0.length() > 0.0 {
            for child in children.iter() {
                if let Ok((mut sprite, _)) = child_query.get_mut(*child) {
                    if velocity.0.x != 0.0 {
                        sprite.flip_x = velocity.0.x < 0.0;
                    }
                }
            }

            AnimationType::Walk
        } else {
            AnimationType::Idle
        };

        if new_animation != current_animation.0 {
            current_animation.0 = new_animation;
            for child in children.iter() {
                if let Ok((_, mut animation_timer)) = child_query.get_mut(*child) {
                    animation_timer.0.reset();
                }
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &LoopAnimation,
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (loop_animation, indices, mut timer, mut atlas) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            atlas.index = if atlas.index == indices.last {
                if loop_animation.0 {
                    timer.reset();
                    indices.first
                } else {
                    indices.last
                }
            } else {
                timer.reset();
                atlas.index + 1
            };
        }
    }
}

pub fn update_animation_visibility(
    query: Query<(&Children, &CurrentAnimation)>,
    mut animation_query: Query<(Entity, &mut Visibility, &AnimationType)>,
) {
    for (children, current_animation) in query.iter() {
        for &child in children.iter() {
            if let Ok((_, mut visibility, animation_type)) = animation_query.get_mut(child) {
                *visibility = if current_animation.0 == *animation_type {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
