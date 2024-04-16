pub mod animation;
pub mod dark_arts_defense;
pub mod player {
    pub mod movement;
    pub mod plugin;
    pub mod spawn;
    pub mod summoning;
}
pub mod units {
    pub mod acolyte;
    pub mod health;
    pub mod team;
    pub mod unit_types;
}
pub mod enemies {
    pub mod enemy_spawner;
    pub mod plugin;
}
pub mod mana;
pub mod movement;
pub mod velocity;
pub mod ai {
    pub mod behavior;
    pub mod plugin;
}
pub mod ui {
    pub mod health_text;
    pub mod mana_text;
    pub mod plugin;
    pub mod score_text;
}
pub mod gamestate;

use bevy::prelude::*;
use bevy::window::{EnabledButtons, WindowMode, WindowResolution};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            dark_arts_defense::DarkArtsDefensePlugin,
        ))
        .add_systems(Startup, setup_window)
        .run();
}

fn setup_window(mut query: Query<&mut Window>) {
    let mut window = query.single_mut();
    window.cursor.visible = false;
    window.mode = WindowMode::BorderlessFullscreen;
    window.resolution = WindowResolution::new(1920.0, 1080.0);
    window.title = "Dark Arts Defense".to_owned();
    window.resize_constraints = WindowResizeConstraints {
        min_width: 1280.0,
        min_height: 720.0,
        max_width: 3840.0,
        max_height: 2160.0,
    };
    window.resizable = true;
    window.enabled_buttons = EnabledButtons {
        minimize: false,
        maximize: false,
        close: false,
    };
    window.decorations = false;
    window.transparent = false;
    window.focused = true;
    window.visible = true;
}
