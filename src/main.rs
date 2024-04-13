pub mod dark_arts_defense;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            dark_arts_defense::DarkArtsDefensePlugin,
        ))
        .run();
}
