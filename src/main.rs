pub mod dark_arts_defense;
pub mod player {
    pub mod movement;
    pub mod plugin;
    pub mod spawn;
}
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            dark_arts_defense::DarkArtsDefensePlugin,
        ))
        .run();
}
