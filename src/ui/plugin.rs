use bevy::prelude::*;

use crate::{dark_arts_defense::GameEvent, gamestate::GameState};

use super::{health_text, mana_text, score_text};

pub struct UiPlugin;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ManaText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameOverText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                update_health_pos,
                update_mana_pos,
                update_score_pos,
                health_text::update_health_text,
                mana_text::update_mana_text,
                score_text::update_mana_text,
                game_over_ui,
            ),
        );
    }
}

const TEXT_OFFSET_TOP: f32 = 0.15;
const TEXT_OFFSET_CENTER: f32 = 0.3;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) {
    let font = asset_server.load("fonts/JetBrainsMonoNerdFont-Regular.ttf");
    let window = window_query.single();
    let window_bounds = Vec2::new(window.width(), window.height()) * 0.5;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "MP: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: Color::BLUE,
                },
            )
            .with_justify(JustifyText::Right),
            transform: Transform {
                translation: Vec3::new(window_bounds.x * TEXT_OFFSET_CENTER, window_bounds.y, 0.0),
                ..default()
            },
            ..default()
        },
        ManaText,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "HP: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: Color::GREEN,
                },
            )
            .with_justify(JustifyText::Left),
            transform: Transform {
                translation: Vec3::new(-window_bounds.x * TEXT_OFFSET_CENTER, window_bounds.y, 0.0),
                ..default()
            },
            ..default()
        },
        HealthText,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform {
                translation: Vec3::new(0.0, -window_bounds.y, 0.0),
                ..default()
            },
            ..default()
        },
        ScoreText,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Game Over\nPress SPACE to restart",
                TextStyle {
                    font: font.clone(),
                    font_size: 90.0,
                    color: Color::WHITE,
                },
            )
            .with_justify(JustifyText::Center),
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText,
    ));
}

fn update_text_pos(window_query: Query<&Window>, transform: &mut Transform, direction: f32) {
    let window = window_query.single();
    let window_bounds = Vec2::new(window.width(), window.height()) * 0.5;

    transform.translation = Vec3::new(
        window_bounds.x * direction * TEXT_OFFSET_CENTER,
        window_bounds.y + -window_bounds.y * TEXT_OFFSET_TOP,
        0.0,
    );
}

fn update_mana_pos(window_query: Query<&Window>, mut query: Query<&mut Transform, With<ManaText>>) {
    update_text_pos(window_query, &mut query.single_mut(), 1.0);
}

fn update_health_pos(
    window_query: Query<&Window>,
    mut query: Query<&mut Transform, With<HealthText>>,
) {
    update_text_pos(window_query, &mut query.single_mut(), -1.0);
}

fn update_score_pos(
    window_query: Query<&Window>,
    mut query: Query<&mut Transform, With<ScoreText>>,
) {
    let window = window_query.single();
    let window_bounds = Vec2::new(window.width(), window.height()) * 0.5;

    let mut transform = query.single_mut();
    transform.translation = Vec3::new(
        0.0,
        -window_bounds.y + window_bounds.y * TEXT_OFFSET_TOP,
        0.0,
    );
}

fn game_over_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut visible_query: Query<&mut Visibility, With<GameOverText>>,
    mut game_state_query: Query<&mut GameState>,
    mut event_writer: EventWriter<GameEvent>,
) {
    for mut game_state in game_state_query.iter_mut() {
        if game_state.end_screen_active {
            for mut visibility in visible_query.iter_mut() {
                *visibility = Visibility::Visible; // Dereference and assign the value
            }

            if keys.just_pressed(KeyCode::Space) {
                game_state.end_screen_active = false;
                *visible_query.single_mut() = Visibility::Hidden;
                event_writer.send(GameEvent::StartGame);
            }
        }
    }
}
