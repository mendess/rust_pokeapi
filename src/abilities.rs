use crate::dex_error::DexError;
use crate::parser::abilities::AbilityJson;
use serde_json::from_reader;

#[derive(Debug)]
pub struct Ability {
    pub name: String,
    pub gen: u16,
    pub short_effect: String,
    pub effect: String,
}


impl Ability{
    pub fn fetch(url: String) -> Result<Ability, DexError> {
        let resp = super::reqwest::get(&url)?;
        let ab_json :AbilityJson = from_reader(resp)?;
        let a = Ability {
            name: ab_json.name,
            gen: gen_str_2_num(ab_json.generation.name),
            short_effect: ab_json.effect_entries
                .iter()
                .find(|x| x.language.name == "en")
                .map(|x| x.short_effect.clone())
                .unwrap_or("none".to_string()),
            effect: ab_json.effect_entries
                .iter()
                .find(|x| x.language.name == "en")
                .map(|x| x.effect.clone())
                .unwrap_or("none".to_string()),
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
