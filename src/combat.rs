use std::collections::HashSet;

use crate::{base::{DamageType, Die}, player::{CoreAbility, CoreSkill}};

pub enum TimeCost {
    Action,
    BonusAction,
    Reaction,
    FreeAction,
}

#[repr(transparent)]
pub struct AttackBonus(usize);

pub struct Damage {
    pub dice: Vec<Die>,
    pub flat: usize,
    pub damage_type: DamageType,
}

pub enum Attack {
    WeaponAttack(TimeCost, AttackBonus, Vec<Damage>),
    SpellAttack(TimeCost, Vec<Damage>),
    Special(SpecialWeapon),
}

pub enum SpecialWeapon {
    DragonbornBreathAttack(DamageType),
}

#[derive(PartialEq, Eq, Hash)]
pub enum Weapon {
    Club,
    Dagger,
    Greatclub,
    Handaxe,
    Javelin,
    LightHammer,
    Mace,
    Quarterstaff,
    Sickle,
    Spear,
    CrossbowLight,
    Dart,
    Shortbow,
    Sling,
    Battleaxe,
    Flail,
    Glaive,
    Greataxe,
    Greatsword,
    Halberd,
    Lance,
    Longsword,
    Maul,
    Morningstar,
    Pike,
    Rapier,
    Scimitar,
    Shortsword,
    Trident,
    WarPick,
    Warhammer,
    Whip,
    Blowgun,
    CrossbowHand,
    CrossbowHeavy,
    Longbow,
    Net,
    Custom(String),
}

impl Weapon {
    pub fn simple_melee() -> HashSet<Self> {
        HashSet::from_iter(vec![
            Self::Club,
            Self::Dagger,
            Self::Greatclub,
            Self::Handaxe,
            Self::Javelin,
            Self::LightHammer,
            Self::Mace,
            Self::Quarterstaff,
            Self::Sickle,
            Self::Spear,
        ])
    }

    pub fn simple_ranged() -> HashSet<Self> {
        HashSet::from_iter(vec![
            Self::CrossbowLight,
            Self::Dart,
            Self::Shortbow,
            Self::Sling,
        ])
    }

    pub fn martial_melee() -> HashSet<Self> {
        HashSet::from_iter(vec![
            Self::Battleaxe,
            Self::Flail,
            Self::Glaive,
            Self::Greataxe,
            Self::Greatsword,
            Self::Halberd,
            Self::Lance,
            Self::Longsword,
            Self::Maul,
            Self::Morningstar,
            Self::Pike,
            Self::Rapier,
            Self::Scimitar,
            Self::Shortsword,
            Self::Trident,
            Self::WarPick,
            Self::Warhammer,
            Self::Whip,
        ])
    }

    pub fn martial_ranged() -> HashSet<Self> {
        HashSet::from_iter(vec![
            Self::Blowgun,
            Self::CrossbowHand,
            Self::CrossbowHeavy,
            Self::Longbow,
            Self::Net,
        ])
    }
}
