use ::parser::pokemon::PkMoveJson;

#[derive(Debug)]
pub struct PkMove {
    name: String,
    learn_method: String,
    level_learned_at: Option<u32>,
}

impl<'a> From<&'a PkMoveJson> for PkMove {
    fn from(mv: &PkMoveJson) -> Self {
        PkMove {
            name: mv.name.name.clone(),
            learn_method: mv.version_group_details[0].move_learn_method.name.clone(),
            level_learned_at: {
                match mv.version_group_details[0].level_learned_at {
                    0 => None,
                    x => Some(x),
                }
            }
        }
    }
}
