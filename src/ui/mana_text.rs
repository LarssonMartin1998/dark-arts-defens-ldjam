use bevy::prelude::*;

use crate::{mana::Mana, player::spawn::Player};

use super::plugin::ManaText;

pub fn update_mana_text(
    query: Query<&Mana, With<Player>>,
    mut text_query: Query<&mut Text, With<ManaText>>,
) {
    let mana = query.single();
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("MP: {}", mana.current_mana);
}
