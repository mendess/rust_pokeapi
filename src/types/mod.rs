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
