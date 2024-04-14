use bevy::prelude::*;

use crate::{player::spawn::Player, units::health::Health};

use super::plugin::HealthText;

pub fn update_health_text(
    query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    let health = query.single();
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("HP: {}", health.0);
}
