use crate::{
    base::{DamageModifier, DamageType, Language}, combat::{Attack, SpecialWeapon}, define_feature_enum, options::{FeatureSelection, FeatureSelections}, player::{BasePlayer, CoreAbility, PlayerOptions}
};

define_feature_enum! {
    pub enum DraconicAncestry {
        Black,
        Blue,
        Brass,
        Bronze,
        Copper,
        Gold,
        Green,
        Red,
        Silver,
        White
    }
}

pub fn modify_player(player: &mut BasePlayer, options: &PlayerOptions) {
    let dragonborn_options = options.options.get(&feature_id());

    let ancestry_id = dragonborn_options.map(|v| v.get(0)).flatten();
    let ancestry = ancestry_id.map(|id| DraconicAncestry::from_id(*id)).flatten().unwrap_or(DraconicAncestry::Black);

    let damage_type = match ancestry {
        DraconicAncestry::Black => DamageType::Acid,
        DraconicAncestry::Blue => DamageType::Lightning,
        DraconicAncestry::Brass => DamageType::Fire,
        DraconicAncestry::Bronze => DamageType::Lightning,
        DraconicAncestry::Copper => DamageType::Acid,
        DraconicAncestry::Gold => DamageType::Fire,
        DraconicAncestry::Green => DamageType::Poison,
        DraconicAncestry::Red => DamageType::Fire,
        DraconicAncestry::Silver => DamageType::Cold,
        DraconicAncestry::White => DamageType::Cold,
    };

    player.attacks.push(Attack::Special(SpecialWeapon::DragonbornBreathAttack(damage_type)));
    player.damage_modifiers.insert(damage_type, DamageModifier::Resistance);
    
    player.languages.insert(Language::Common);
    player.languages.insert(Language::Draconic);
}

pub fn feature_id() -> usize {
    0b0000000100011100111000101001101010111111000000001010100001011100
}

pub fn selections() -> FeatureSelections {
    FeatureSelections {
        selections: vec![DraconicAncestry::to_feature_selection()],
    }
}
