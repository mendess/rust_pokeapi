
#[derive(Deserialize)]
pub struct AbilityJson {
    pub name: String,
    pub generation: GenJson,
    pub effect_entries: Vec<EffectEntryJson>,
}

#[derive(Deserialize)]
pub struct GenJson {
    pub url: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct EffectEntryJson {
    pub short_effect: String,
    pub effect: String,
    pub language: EffectLangJson,
}

#[derive(Deserialize)]
pub struct EffectLangJson {
    pub url: String,
    pub name: String,
}
