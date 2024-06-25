pub mod subclass;

use crate::spells::{cantrips::Cantrip, spells::Spell, spellslot::SpellSlot};

pub struct Feature {
    pub name: String,
}

pub struct CantripChoice {
    pub cantrips: Vec<Cantrip>,
    pub number: u32,
}

pub struct SpellChoice {
    pub spells: Vec<Spell>,
    pub number_of_spells: u32,
}

pub struct FeatureChoice {
    pub features: Vec<Feature>,
    pub number_of_features: u32,
}

pub struct NewAbilities {
    pub cantrip_choice: Option<CantripChoice>,
    pub spell_choice: SpellChoice,
    pub subclass_choice: Option<Vec<subclass::SubClass>>,
    pub spell_slots: Vec<SpellSlot>,
}

pub trait Levelling {
    fn level1() -> NewAbilities;
    fn level2() -> NewAbilities;
}
