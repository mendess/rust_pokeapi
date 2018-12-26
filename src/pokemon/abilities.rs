use crate::abilities::Ability;
use crate::parser::pokemon::*;

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
                Ok(a) => a.short_effect,
                Err(_) => String::from("none"),
            }
        }
    }
}
