mod moves;
mod forms;
pub mod types;
pub mod abilities;
pub mod base_stats;

use pokemon::types::PkType;
use pokemon::abilities::PkAbility;
use pokemon::base_stats::PkBaseStats;
use pokemon::moves::PkMove;
use pokemon::forms::PkForm;
use ::dex_error::DexError;
use ::parser::pokemon::PokemonJson;
use ::parser::NamedAPIResource;

use serde_json::from_reader;

#[derive(Debug)]
pub struct Pokemon {
    id: u32,
    name: String,
    height: u32,
    weight: u32,
    abilities: Vec<PkAbility>,
    forms: Vec<Form>,
    types: Vec<PkType>,
    base_stats: PkBaseStats,
    moves: Vec<PkMove>,
}

#[derive(Debug)]
enum Form {
    Reference(NamedAPIResource),
    Value(PkForm),
}

impl<'a> From<&'a NamedAPIResource> for Form {
    fn from(n: &NamedAPIResource) -> Self {
        Form::Reference(NamedAPIResource {
            name: n.name.clone(),
            url: n.url.clone(),
        })
    }
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
            forms: pk.forms.iter().map(|f| Form::from(f)).collect(),
            base_stats: PkBaseStats::from(pk.stats),
            moves: pk.moves.iter().map(|x| PkMove::from(x)).collect(),
        }
    }
}

fn url_fetch(url: &str) -> Result<Pokemon, DexError> {
    let resp = super::reqwest::get(url)?;
    if resp.status().is_success() {
        let pk :PokemonJson = from_reader(resp)?;
        Ok(Pokemon::from(pk))
    }else{
        Err(DexError::Other(format!("{:?}", resp.status())))
    }
}

impl Pokemon {
    pub fn fetch<T: super::std::fmt::Display>(id: T) -> Result<Pokemon, DexError> {
        url_fetch(&format!("https://pokeapi.co/api/v2/pokemon/{}/", id))
    }

    pub fn number(&self) -> u32{
        self.id
    }

    pub fn name(&self) -> &String{
        &self.name
    }

    pub fn height(&self) -> u32{
        self.height
    }

    pub fn weight(&self) -> u32{
        self.weight
    }

    pub fn abilities(&self) -> &Vec<PkAbility>{
        &self.abilities
    }

    pub fn types(&self) -> &Vec<PkType>{
        &self.types
    }

    pub fn base_stats(&self) -> &PkBaseStats{
        &self.base_stats
    }

    pub fn moves(&self) -> &Vec<PkMove> {
        &self.moves
    }
}
