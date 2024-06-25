use crate::levelling::subclass::SubClass;
use crate::levelling::{CantripChoice, Levelling, NewAbilities, SpellChoice};
use crate::spells::{cantrips::Cantrip, spells::Spell, spellslot::RegainOn, spellslot::SpellSlot};
use core::fmt;

pub struct Warlock {}

impl fmt::Display for Warlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a warlock")
    }
}

impl Levelling for Warlock {
    fn level1() -> NewAbilities {
        NewAbilities {
            cantrip_choice: Some(CantripChoice {
                cantrips: vec![
                    Cantrip::BladeWard,
                    Cantrip::BoneChill,
                    Cantrip::EldritchBlast,
                    Cantrip::Friends,
                    Cantrip::MageHand,
                    Cantrip::MinorIllusion,
                    Cantrip::PoisonSpray,
                    Cantrip::TrueStrike,
                ],
                number: 2,
            }),
            spell_choice: SpellChoice {
                spells: vec![
                    Spell::ArmourOfAgathys,
                    Spell::ArmsOfHadar,
                    Spell::CharmPerson,
                    Spell::ExpeditiousRetreat,
                    Spell::HellishRebuke,
                    Spell::Hex,
                    Spell::ProtectionFromEvilAndGood,
                    Spell::WitchBolt,
                ],
                number_of_spells: 2,
            },
            subclass_choice: Some(vec![
                SubClass::TheFiend,
                SubClass::TheGreatOldOne,
                SubClass::TheArchfey,
            ]),
            spell_slots: vec![SpellSlot {
                level: 1,
                number_of_spells: 1,
                regain_on: RegainOn::ShortRest,
            }],
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
            spell_slots: vec![SpellSlot {
                level: 1,
                number_of_spells: 2,
                regain_on: RegainOn::ShortRest,
            }],
        }
    }
}
