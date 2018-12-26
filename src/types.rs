#[derive(Debug)]
pub enum TypeName {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
}

impl From<String> for TypeName {
    fn from(string: String) -> Self {
        use self::TypeName::*;
        match string.as_ref() {
            "normal" => Normal,
            "fire" => Fire,
            "fighting" => Fighting,
            "water" => Water,
            "flying" => Flying,
            "grass" => Grass,
            "poison" => Poison,
            "electric" => Electric,
            "ground" => Ground,
            "psychic" => Psychic,
            "rock" => Rock,
            "ice" => Ice,
            "bug" => Bug,
            "dragon" => Dragon,
            "ghost" => Ghost,
            "dark" => Dark,
            "steel" => Steel,
            "fairy" => Fairy,
            invalid => panic!("invalid pokemon type: {}", invalid)
        }
    }
}

pub struct Type {
    id: i32,
    name: TypeName,
    dmg_relations: DmgRelations,
}

pub struct DmgRelations {
    no_dmg_from: Vec<TypeName>,
    hf_dmg_from: Vec<TypeName>,
    db_dmg_from: Vec<TypeName>,
    no_dmg_to: Vec<TypeName>,
    hf_dmg_to: Vec<TypeName>,
    db_dmg_to: Vec<TypeName>,
}

impl Type {
    pub fn normal() -> Type{
        use self::TypeName::*;
        Type {
            id: 1,
            name: Normal,
            dmg_relations: DmgRelations {
                no_dmg_from: vec![Ghost],
                hf_dmg_from: vec![],
                db_dmg_from: vec![Fighting],
                no_dmg_to: vec![Ghost],
                hf_dmg_to: vec![Rock, Steel],
                db_dmg_to: vec![]
            }
        }
    }
}
