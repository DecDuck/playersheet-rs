use crate::impl_mut_player;

impl_mut_player! {
    pub enum Race {
        // Common
        Dragonborn,
        Dwarf,
        Elf,
        Gnome,
        HalfElf,
        HalfOrc,
        Halfing,
        Human,
        Tiefling,
        // Exotic
        Aarakocra,
        Aasimar,
        Changeling,
        DeepGnome,
        Duergar,
        Eladrin,
        Fairy,
        Firbolg,
        AirGenasi,
        EarthGenasi,
        FireGenasi,
        WaterGenasi,
        Githyanki,
        Githzerai,
        Goliath,
        Harengon,
        Kenku,
        Locathah,
        Owlin,
        Satyr,
        SeaElf,
        ShaderKai,
        Tabaxi,
        Tortle,
        Triton,
        Verdan,
        // Monstrous
        Bugbear,
        Centaur,
        Goblin,
        Grung,
        Hobgobling,
        Kobold,
        Lizardfolk,
        Minotaur,
        Orc,
        Shifter,
        YuanTi,
        // Setting specific
        Kender,
        Kalashtar,
        Warforged,
        Aetherborn,
        Aven,
        Khenra,
        Kor,
        Merfolk,
        Naga,
        Siren,
        Vampire,
        Dhampir,
        Hexblood,
        Reborn,
        Loxodon,
        SimicHybrid,
        Vedalken,
        AstralElf,
        Autognome,
        Giff,
        Hadozee,
        Plasmoid,
        Thrikreen,
        Leonin,
    }

    module = race;

    // The common block now receives the options as well
    common(_race, p, _opts) => {
        
    }

}
