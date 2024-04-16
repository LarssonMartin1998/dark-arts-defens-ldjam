use bevy::prelude::*;

use crate::gamestate::GameState;

use super::plugin::ScoreText;

pub fn update_mana_text(
    query: Query<&GameState>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
) {
    if let Some(gamestate) = query.iter().next() {
        let mut text = text_query.single_mut();
        text.sections[0].value = format!("Score: {}", gamestate.score);
    }
}
