pub enum ClassType {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Ranger,
    Rogue,
    Paladin,
    Sorcerer,
    Warlock,
    Wizard,
}

#[repr(transparent)]
pub struct ClassLevel(usize);

pub struct Class {
    pub class_type: ClassType,
    pub class_level: ClassLevel,
}