use std::{collections::{HashMap, HashSet}, mem};

use crate::{
    base::{DamageModifier, DamageType, Language, SaveModifier, SavingThrowSource, Size, SpeedType},
    class::Class,
    combat::{Attack, Weapon}, tools::Tool,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CoreAbility {
    Strength,
    Dexerity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(PartialEq, Eq, Hash)]
pub enum CoreSkill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

pub struct BasePlayer {
    pub abilities: HashMap<CoreAbility, isize>,
    pub skill_modifiers: HashMap<CoreSkill, usize>,
    
    pub size: Size,
    pub speeds: HashMap<SpeedType, usize>,
    pub max_health: usize,

    pub darkvision: Option<Darkvision>,
    pub weapon_proficiencies: HashSet<Weapon>,
    pub tool_proficienies: HashSet<Tool>,

    pub damage_modifiers: HashMap<DamageType, DamageModifier>,
    pub attacks: Vec<Attack>,
    pub saving_throw_mods: HashMap<SavingThrowSource, SaveModifier>,

    pub languages: HashSet<Language>,

    pub levels: Vec<Class>,
}

impl BasePlayer {
    pub fn apply_asi(&mut self, asi: Vec<(CoreAbility, isize)>) {
        for (abi, amount) in asi {
            let base = *self.abilities.get(&abi).unwrap_or(&0);
            let next = base + amount;
            self.abilities.insert(abi, next);
        }
    }

    pub fn overwrite_speeds(&mut self, speeds: Vec<(SpeedType, usize)>) {
        for (speed_type, amount) in speeds {
            self.speeds.insert(speed_type, amount);
        }
    }

    pub fn add_darkvision(&mut self, darkvision: Darkvision) {
        if let Some(existing_darkvision) = &mut self.darkvision {
            if existing_darkvision.distance < darkvision.distance {
                mem::replace(existing_darkvision, darkvision);
            }
        } else {
            self.darkvision = Some(darkvision);
        }
    }
}

pub struct PlayerOptions {
    pub options: HashMap<usize, Vec<usize>>,
}

pub trait MutPlayer {
    fn modify(&self, player: &mut BasePlayer, options: &PlayerOptions);
}

pub struct Darkvision {
    pub distance: usize,
    pub colour: bool,
}
