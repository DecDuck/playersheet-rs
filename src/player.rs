use std::collections::HashMap;

pub enum CoreAbility {
    Strength,
    Dexerity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

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
    pub abilities: HashMap<CoreAbility, usize>,
    pub skill_modifiers: HashMap<CoreSkill, usize>,
}

pub struct PlayerOptions {
    pub options: HashMap<usize, Vec<usize>>,
}

pub trait MutPlayer {
    fn modify(&self, player: &mut BasePlayer, options: &PlayerOptions);
}