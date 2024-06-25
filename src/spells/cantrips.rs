pub enum Cantrip {
    BladeWard,
    BoneChill,
    EldritchBlast,
    Friends,
    MageHand,
    MinorIllusion,
    PoisonSpray,
    TrueStrike,
}

#[allow(dead_code)]
impl Cantrip {
    fn name(&self) -> String {
        match self {
            Cantrip::BladeWard => "Blade Ward".to_string(),
            Cantrip::BoneChill => "Bone Chill".to_string(),
            Cantrip::EldritchBlast => "Eldritch Blast".to_string(),
            Cantrip::Friends => "Friends".to_string(),
            Cantrip::MageHand => "Mage Hand".to_string(),
            Cantrip::MinorIllusion => "Minor Illusion".to_string(),
            Cantrip::PoisonSpray => "Poison Spray".to_string(),
            Cantrip::TrueStrike => "True Strike".to_string(),
        }
    }
}
