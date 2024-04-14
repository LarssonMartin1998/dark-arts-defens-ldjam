use crate::animation::spawn_animated_children;
use crate::mana::Mana;
use crate::player::spawn::Player;
use crate::units::unit_types::{
    Acolyte, Cat, UnitChildrenSpawnParamsFactory, UnitResource, UnitType, Warrior,
};
use bevy::prelude::*;

pub fn system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    keys: Res<ButtonInput<KeyCode>>,
    unit_configs: Res<UnitResource>,
    mut query: Query<(&mut Mana, &Transform), With<Player>>,
) {
    let column_staggered_colemak_binds = vec![
        (KeyCode::KeyN, UnitType::Acolyte),
        (KeyCode::KeyE, UnitType::Warrior),
        (KeyCode::KeyI, UnitType::Cat),
    ];
    let pressed_units = handle_input(&keys, &column_staggered_colemak_binds);

    // let row_staggered_qwerty_binds = vec![
    //     (KeyCode::Digit1, UnitType::Acolyte),
    //     (KeyCode::Digit2, UnitType::Warrior),
    //     (KeyCode::Digit3, UnitType::Cat),
    // ];
    // let pressed_units = handle_input(&keys, &row_staggered_qwerty_binds);

    pressed_units.into_iter().for_each(|(_, unit)| {
        let (mut mana, transform) = query.single_mut();
        let unit_cost = unit_configs.get(*unit).cost;
        if mana.0 < unit_cost {
            return;
        }

        match unit {
            UnitType::Acolyte => summon_unit(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                Acolyte,
                transform,
            ),
            UnitType::Warrior => summon_unit(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                Warrior,
                transform,
            ),
            UnitType::Cat => summon_unit(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                Cat,
                transform,
            ),
        }

        mana.0 -= unit_cost;
        println!("Mana: {}", mana.0);
    });
}

fn handle_input<'a>(
    keys: &'a Res<ButtonInput<KeyCode>>,
    binds: &'a [(KeyCode, UnitType)],
) -> impl Iterator<Item = &'a (KeyCode, UnitType)> + 'a {
    binds
        .iter()
        .filter(move |(key, _unit)| keys.just_pressed(*key))
}

fn summon_unit(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    unit_component: impl UnitChildrenSpawnParamsFactory,
    player_transform: &Transform,
) {
    let mut bundle = unit_component.create_bundle();
    bundle.transform.translation = player_transform.translation;
    commands.spawn(bundle).with_children(|parent| {
        spawn_animated_children(
            asset_server,
            texture_atlas_layouts,
            parent,
            unit_component.create_children_spawn_params(),
        );
    });
}
