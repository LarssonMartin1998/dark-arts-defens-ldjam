use bevy::prelude::*;

use crate::{mana::Mana, player::spawn::Player};
use crate::units::health::Health;

use super::unit_types::Acolyte;

pub fn acolyte_mana_giver(
    time: Res<Time>,
    mut query: Query<(&mut Acolyte, &Health)>,
    mut player_query: Query<&mut Mana, With<Player>>,
) {
    for (mut acolyte, health) in query.iter_mut() {
        if health.is_dead() {
            continue;
        }

        if acolyte.give_mana_timer.tick(time.delta()).just_finished() {
            let mut mana = player_query.single_mut();
            mana.current_mana = (mana.current_mana + acolyte.mana_amount).min(mana.max_mana);
        }
    }
}
