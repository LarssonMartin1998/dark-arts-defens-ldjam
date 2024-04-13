use crate::mana::Mana;
use crate::player::spawn::Player;
// use crate::units::unit_types::AoeCaster;
// use crate::units::unit_types::Harvester;
// use crate::units::unit_types::SingleTargetCaster;
// use crate::units::unit_types::Tank;
use crate::units::unit_types::UnitResource;
use crate::units::unit_types::UnitType;
use bevy::prelude::*;

pub fn system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    unit_costs: Res<UnitResource>,
    mut query: Query<&mut Mana, With<Player>>,
) {
    let column_staggered_colemak_binds = [
        (KeyCode::KeyN, UnitType::Harvester),
        (KeyCode::KeyE, UnitType::Tank),
        (KeyCode::KeyI, UnitType::AoeCaster),
        (KeyCode::KeyO, UnitType::SingleTargetCaster),
    ];
    handle_summoning(
        &commands,
        &asset_server,
        keys,
        &texture_atlas_layouts,
        &unit_costs,
        &mut query,
        column_staggered_colemak_binds,
    );

    // let row_staggered_qwerty_binds = [
    //     (KeyCode::Digit1, UnitType::Harvester),
    //     (KeyCode::Digit2, UnitType::Tank),
    //     (KeyCode::Digit3, UnitType::AoeCaster),
    //     (KeyCode::Digit4, UnitType::SingleTargetCaster)
    // ];
    // handle_summoning(keys, column_staggered_colemak_binds, mana, query);
}

fn handle_summoning(
    commands: &Commands,
    asset_server: &Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    texture_atlas_layouts: &ResMut<Assets<TextureAtlasLayout>>,
    unit_costs: &Res<UnitResource>,
    query: &mut Query<&mut Mana, With<Player>>,
    binds: [(KeyCode, UnitType); 4],
) {
    for (key, unit) in binds.iter() {
        if keys.just_pressed(*key) {
            try_summon_unit(
                commands,
                asset_server,
                texture_atlas_layouts,
                unit_costs,
                query,
                *unit,
            );
        }
    }
}

fn try_summon_unit(
    commands: &Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &ResMut<Assets<TextureAtlasLayout>>,
    unit_configs: &Res<UnitResource>,
    query: &mut Query<&mut Mana, With<Player>>,
    unit: UnitType,
) {
    let mut mana = query.single_mut();
    let unit_cost = unit_configs.get(unit).cost;
    if mana.0 < unit_cost {
        return;
    }

    // match unit {
    //     UnitType::Harvester => {
    //         summon_unit::<Harvester>(commands, asset_server, texture_atlas_layouts)
    //     }
    //     UnitType::Tank => summon_unit::<Tank>(commands, asset_server, texture_atlas_layouts),
    //     UnitType::AoeCaster => {
    //         summon_unit::<AoeCaster>(commands, asset_server, texture_atlas_layouts)
    //     }
    //     UnitType::SingleTargetCaster => {
    //         summon_unit::<SingleTargetCaster>(commands, asset_server, texture_atlas_layouts)
    //     }
    // }
    mana.0 -= unit_cost;
    println!("Mana: {}", mana.0);
}

// fn summon_unit<T>(
//     commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) where
//     T: Component,
// {
//     let texture = asset_server.load("player/player_idle.png");
//     let layout = TextureAtlasLayout::from_grid(Vec2::new(80.0, 80.0), 3, 4, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);
//     // Use only the subset of sprites in the sheet that make up the run animation
//     let animation_indices = AnimationIndices { first: 0, last: 9 };
//
//     commands.spawn((
//         SpriteSheetBundle {
//             texture,
//             atlas: TextureAtlas {
//                 layout: texture_atlas_layout,
//                 index: animation_indices.first,
//             },
//             transform: Transform::from_scale(Vec3::splat(1.5)),
//             ..default()
//         },
//         animation_indices,
//         AnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
//         T,
//     ));
// }
