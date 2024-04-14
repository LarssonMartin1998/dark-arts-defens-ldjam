use bevy::prelude::*;

use crate::enemies::enemy_spawner;

pub struct EnemyPlugin;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(200.0, TimerMode::Repeating)))
            .add_systems(Update, enemy_spawner::spawn_enemies);
    }
}
