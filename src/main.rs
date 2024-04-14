pub mod animation;
pub mod dark_arts_defense;
pub mod player {
    pub mod movement;
    pub mod plugin;
    pub mod spawn;
    pub mod summoning;
}
pub mod units {
    pub mod unit_types;
}
pub mod mana;
pub mod movement;
pub mod velocity;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            dark_arts_defense::DarkArtsDefensePlugin,
        ))
        .run();
}
