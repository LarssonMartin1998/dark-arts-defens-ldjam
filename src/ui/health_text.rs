use bevy::prelude::*;

use crate::{player::plugin::Player, units::health::Health};

use super::plugin::HealthText;

pub fn update_health_text(
    query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    if let Some(health) = query.iter().next() {
        let mut text = text_query.single_mut();
        text.sections[0].value = format!("HP: {}", health.0);
    }
}
