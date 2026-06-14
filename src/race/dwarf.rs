use crate::{
    base::{DamageModifier, DamageType, SaveModifier, SavingThrowSource},
    combat::Weapon,
    define_feature_enum,
    options::FeatureSelections,
    player::{BasePlayer, Darkvision, PlayerOptions},
    tools::Tool::{self, Mason},
};

define_feature_enum! {
    pub enum DwarfType {
        Hill,
        Mountain,
        MarkOfWarding,
        Planeshift
    }
}

define_feature_enum! {
    pub enum ToolProficiency {
        Smith,
        Brewer,
        Mason
    }
}

pub fn modify_player(player: &mut BasePlayer, options: &PlayerOptions) {
    let dwarf_options = options.options.get(&feature_id()).unwrap();

    player.add_darkvision(Darkvision {
        distance: 60,
        colour: false,
    });
    player.saving_throw_mods.insert(
        SavingThrowSource::Custom(format!("poison")),
        SaveModifier::Advantage,
    );
    player
        .damage_modifiers
        .insert(DamageType::Poison, DamageModifier::Resistance);
    player.weapon_proficiencies.extend(vec![
        Weapon::Battleaxe,
        Weapon::Handaxe,
        Weapon::LightHammer,
        Weapon::Warhammer,
    ]);

    let tool_prof = dwarf_options.get(1).unwrap();
    let tool_prof = ToolProficiency::from_id(*tool_prof).unwrap();
    player.tool_proficienies.insert(match tool_prof {
        ToolProficiency::Smith => Tool::Smith,
        ToolProficiency::Brewer => Tool::Brewer,
        ToolProficiency::Mason => Tool::Mason,
    });

    let dwarf_type = dwarf_options.get(0).unwrap();
    let dwarf_type = DwarfType::from_id(*dwarf_type).unwrap();

    match dwarf_type {
        DwarfType::Hill => todo!(),
        DwarfType::Mountain => todo!(),
        DwarfType::MarkOfWarding => todo!(),
        DwarfType::Planeshift => todo!(),
    }
}

pub fn feature_id() -> usize {
    0b0001110111111111111011011000001100100100000101011110001110010000
}

pub fn selections() -> FeatureSelections {
    FeatureSelections {
        selections: vec![
            DwarfType::to_feature_selection(),
            ToolProficiency::to_feature_selection(),
        ],
    }
}
