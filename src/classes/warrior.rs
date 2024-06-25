use core::fmt;

use crate::levelling::{Levelling, NewAbilities, SpellChoice};

pub struct Warrior {}

impl fmt::Display for Warrior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a warrior")
    }
}

impl Levelling for Warrior {
    fn level1() -> NewAbilities {
        NewAbilities {
            cantrip_choice: None,
            spell_choice: SpellChoice {
                spells: vec![],
                number_of_spells: 0,
            },
            subclass_choice: None,
            spell_slots: vec![],
        }
    }

    fn level2() -> NewAbilities {
        NewAbilities {
            cantrip_choice: None,
            spell_choice: SpellChoice {
                spells: vec![],
                number_of_spells: 0,
            },
            subclass_choice: None,
            spell_slots: vec![],
        }
    }
}
