pub enum Spell {
    ArmourOfAgathys,
    ArmsOfHadar,
    CharmPerson,
    ExpeditiousRetreat,
    HellishRebuke,
    Hex,
    ProtectionFromEvilAndGood,
    WitchBolt,
}

impl Spell {
    pub fn name(&self) -> String {
        match self {
            Spell::ArmourOfAgathys => "Armour of Agathys".to_string(),
            Spell::ArmsOfHadar => "Arms of Hadar".to_string(),
            Spell::CharmPerson => "Charm Person".to_string(),
            Spell::ExpeditiousRetreat => "Expeditious Retreat".to_string(),
            Spell::HellishRebuke => "Hellish Rebuke".to_string(),
            Spell::Hex => "Hex".to_string(),
            Spell::ProtectionFromEvilAndGood => "Protection from Evil and Good".to_string(),
            Spell::WitchBolt => "Witch Bolt".to_string(),
        }
    }

    pub fn level(&self) -> u32 {
        match self {
            Spell::ArmourOfAgathys => 1,
            Spell::ArmsOfHadar => 1,
            Spell::CharmPerson => 1,
            Spell::ExpeditiousRetreat => 1,
            Spell::HellishRebuke => 1,
            Spell::Hex => 1,
            Spell::ProtectionFromEvilAndGood => 1,
            Spell::WitchBolt => 1,
        }
    }
}
