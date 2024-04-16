use bevy::prelude::*;

use crate::{mana::Mana, player::plugin::Player};

use super::plugin::ManaText;

pub fn update_mana_text(
    query: Query<&Mana, With<Player>>,
    mut text_query: Query<&mut Text, With<ManaText>>,
) {
    if let Some(mana) = query.iter().next() {
        let mut text = text_query.single_mut();
        text.sections[0].value = format!("MP: {}", mana.current_mana);
    }
}
