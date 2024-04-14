use bevy::prelude::*;

use super::{health_text, mana_text};

pub struct UiPlugin;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ManaText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                update_health_pos,
                update_mana_pos,
                health_text::update_health_text,
                mana_text::update_mana_text,
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
