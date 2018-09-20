use ::dex_error::DexError;

#[derive(Debug)]
pub struct Ability {
    pub name: String,
    pub gen: u16,
    pub effect: String,
}

#[derive(Deserialize)]
pub struct AbilityJson {
    name: String,
    generation: GenJson,
    effect_entries: Vec<EffectEntryJson>,
}

#[derive(Deserialize)]
struct GenJson {
    url: String,
    name: String,
}

#[derive(Deserialize)]
struct EffectEntryJson {
    short_effect: String,
    effect: String,
    language: EffectLangJson,
}

#[derive(Deserialize)]
struct EffectLangJson {
    url: String,
    name: String,
}

impl Ability{
    pub fn fetch(url: String) -> Result<Ability, DexError> {
        let resp = super::reqwest::get(&url)?;
        let ab_json :AbilityJson = super::serde_json::from_reader(resp)?;
        let a = Ability {
            name: ab_json.name,
            gen: gen_str_2_num(ab_json.generation.name),
            effect: ab_json.effect_entries
                .iter()
                .find(|x| x.language.name == "en")
                .map(|x| x.effect.clone())
                .unwrap_or("none".to_string())
        };
        Ok(a)
    }
}

fn gen_str_2_num(string: String) -> u16 {
    match string.as_ref() {
        "generation-i" => 1,
        "generation-ii" => 2,
        "generation-iii" => 3,
        "generation-iv" => 4,
        "generation-v" => 5,
        "generation-vi" => 6,
        "generation-vii" => 7,
        "generation-viii" => 8,
        "generation-ix" => 9,
        "generation-x" => 10,
        _ => 0,
    }
}
