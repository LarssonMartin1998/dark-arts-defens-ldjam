use crate::{mana::Mana, player::movement::PlayerMovement};

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AnimationType {
    #[default]
    Idle,
    Walk,
    Hit,
    Death,
}

#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut, Default, Clone)]
pub struct AnimationTimer(Timer);

#[derive(Component, Debug, Clone, PartialEq, Eq)]
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
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            Player,
            PlayerMovement { speed: 150.0 },
            Mana(100),
            GlobalTransform::default(),
            Transform::from_scale(Vec3::splat(2.0)),
            CurrentAnimation(AnimationType::Idle),
        ))
        .with_children(|parent| {
            let anim_data = [
                ("player/player_idle.png", (50, 1), 49, AnimationType::Idle),
                ("player/player_walk.png", (10, 1), 9, AnimationType::Walk),
                ("player/player_hit.png", (9, 1), 8, AnimationType::Hit),
                ("player/player_death.png", (52, 1), 51, AnimationType::Death),
            ];

            anim_data
                .into_iter()
                .for_each(|(texture_path, grid, last_index, animation_type)| {
                    let layout = TextureAtlasLayout::from_grid(
                        Vec2::new(96.0, 96.0),
                        grid.0,
                        grid.1,
                        None,
                        None,
                    );

                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices {
                        first: 0,
                        last: last_index,
                    };

                    parent.spawn(AnimationBundle {
                        texture: asset_server.load(texture_path),
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout,
                            index: animation_indices.first,
                        },
                        transform: Transform::default(),
                        animation_indices,
                        animation_timer: AnimationTimer(Timer::from_seconds(
                            0.1,
                            TimerMode::Repeating,
                        )),
                        animation_type,
                        ..Default::default()
                    });
                });
        });
}

pub fn change_animation_state(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CurrentAnimation, With<Player>>,
) {
    let new_animation = {
        if keys.just_pressed(KeyCode::KeyZ) {
            Some(AnimationType::Idle)
        } else if keys.just_pressed(KeyCode::KeyX) {
            Some(AnimationType::Walk)
        } else if keys.just_pressed(KeyCode::KeyC) {
            Some(AnimationType::Death)
        } else if keys.just_pressed(KeyCode::KeyD) {
            Some(AnimationType::Hit)
        } else {
            None
        }
    };

    let mut current_animation = query.single_mut();
    if let Some(animation) = new_animation {
        *current_animation = CurrentAnimation(animation);
    }
}

pub fn update_animation_visibility(
    query: Query<(Entity, &Children, &CurrentAnimation), With<Player>>,
    mut animation_query: Query<(Entity, &mut Visibility, &AnimationType)>,
) {
    for (_parent_entity, children, current_animation) in query.iter() {
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
