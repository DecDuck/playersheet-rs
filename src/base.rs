use crate::player::{CoreAbility, CoreSkill};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum DamageModifier {
    Vulernability,
    Resistance,
    Immunity,
    None,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum SpeedType {
    Walking,
    Flying,
    Swimming,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Language {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfing,
    Orc,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
}

pub enum SaveModifier {
    Advantage,
    Disadvantage,
}

#[derive(PartialEq, Eq, Hash)]
pub enum SavingThrowSource {
    Ability(CoreAbility),
    Skill(CoreSkill),
    Custom(String),
}
