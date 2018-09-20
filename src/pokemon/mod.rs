mod types;
mod abilities;
mod base_stats;

use pokemon::types::{PkType, PkTypeJson};
use pokemon::abilities::{PkAbility, PkAbilityJson};
use pokemon::base_stats::{PkBaseStats, PkBaseStatJson};
use ::dex_error::DexError;

use serde_json::from_reader;

#[derive(Debug)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub abilities: Vec<PkAbility>,
    pub types: Vec<PkType>,
    pub base_stats: PkBaseStats,
}

impl From<PokemonJson> for Pokemon {
    fn from(pk: PokemonJson) -> Self {
        Pokemon {
            id: pk.id,
            name: pk.name,
            height: pk.height,
            weight: pk.weight,
            types: {
                let mut v: Vec<PkType> = pk.types.iter().map(|x| PkType::from(x)).collect();
                v.sort_by_key(|x| x.slot);
                v
            },
            abilities: pk.abilities.iter().map(|x| PkAbility::from(x)).collect(),
            base_stats: PkBaseStats::from(pk.stats),
        }
    }
}

#[derive(Deserialize)]
pub struct PokemonJson {
    id: u32,
    name: String,
    weight: u32,
    height: u32,
    types: Vec<PkTypeJson>,
    abilities: Vec<PkAbilityJson>,
    stats: Vec<PkBaseStatJson>,
}

use std::io::Read;

impl Pokemon {
    pub fn fetch(name: String) -> Result<Pokemon, DexError> {
        use reqwest::StatusCode;
        let url = String::from(format!("https://pokeapi.co/api/v2/pokemon/{}/", name));
        let resp = super::reqwest::get(&url)?;
        match resp.status() {
            StatusCode::Ok => {
                let pk :PokemonJson = from_reader(resp)?;
                Ok(Pokemon::from(pk))
            },
            s => Err(DexError::Other(format!("{:?}", s))),
        }
    }
}
