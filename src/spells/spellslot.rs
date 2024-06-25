pub struct SpellSlot {
    pub level: u32,
    pub number_of_spells: u32,
    pub regain_on: RegainOn,
}

pub enum RegainOn {
    ShortRest,
    LongRest,
}
