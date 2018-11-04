use ::abilities::Ability;
use ::parser::pokemon::*;

#[derive(Debug)]
pub struct PkAbility {
    pub name: String,
    pub hidden: bool,
    pub effect: String,
}

impl<'a> From<&'a PkAbilityJson> for PkAbility {
    fn from(ab: &PkAbilityJson) -> Self{
        PkAbility {
            name: ab.ability.name.clone(),
            hidden: ab.is_hidden,
            effect: match Ability::fetch(ab.ability.url.clone()) {
                Ok(a) => a.effect,
                Err(_) => String::from("none"),
            }
        }
    }
}
