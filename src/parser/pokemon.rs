use parser::NamedAPIResource;

#[derive(Deserialize)]
pub struct PokemonJson {
    pub id: u32,
    pub name: String,
    pub base_experience: u32,
    pub height: u32,
    pub weight: u32,
    pub is_default: bool,
    pub order: u32,
    pub abilities: Vec<PkAbilityJson>,
    pub forms: Vec<NamedAPIResource>,
    //pub game_indices:
    pub held_items: Vec<PkHeldItemJson>,
    pub location_area_encounters: String,
    pub moves: Vec<PkMoveJson>,
    //pub sprites: PkSprites,
    pub species: NamedAPIResource,
    pub types: Vec<PkTypeJson>,
    pub stats: Vec<PkBaseStatJson>,
}

#[derive(Deserialize)]
pub struct PkHeldItemJson {
    pub item: NamedAPIResource,
    pub version_details: Vec<PkHeldItemVersionJson>,
}

#[derive(Deserialize)]
pub struct PkHeldItemVersionJson {
    pub version: NamedAPIResource,
    pub rarity: u32,
}

#[derive(Deserialize)]
pub struct PkMoveJson {
    #[serde(rename= "move")]
    pub name: NamedAPIResource,
    pub version_group_details: Vec<PkMoveVersion>,
}

#[derive(Deserialize)]
pub struct PkMoveVersion {
    pub move_learn_method: NamedAPIResource,
    pub version_group: NamedAPIResource,
    pub level_learned_at: u32,
}

#[derive(Deserialize)]
pub struct PkSprites {
    pub front_default: String,
    pub front_shiny: String,
    pub front_female: String,
    pub front_shiny_female: String,
    pub back_default: String,
    pub back_shiny: String,
    pub back_female: String,
    pub back_shiny_female: String,
}

#[derive(Deserialize)]
pub struct PkAbilityJson {
    pub slot: u32,
    pub is_hidden: bool,
    pub ability: PkAbilityNameJson,
}

#[derive(Deserialize)]
pub struct PkAbilityNameJson {
    pub url: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct PkBaseStatJson {
    pub stat: PkStatJson,
    pub effort: u32,
    pub base_stat: u32,
}

#[derive(Deserialize)]
pub struct PkStatJson {
    pub url: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct PkTypeJson {
    pub slot: u16,
    #[serde(rename = "type")]
    pub type_name: PkTpJson,
}

#[derive(Deserialize)]
pub struct PkTpJson {
    pub url: String,
    pub name: String,
}
