use crate::parser::pokemon::PkBaseStatJson;

#[derive(Debug)]
pub struct PkBaseStats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

impl PkBaseStats {
    pub fn total(&self) -> u32 {
        self.hp
            + self.attack
            + self.defense
            + self.sp_attack
            + self.sp_defense
            + self.speed
    }
}

impl From<Vec<PkBaseStatJson>> for PkBaseStats {
    fn from(j: Vec<PkBaseStatJson>) -> Self {
        PkBaseStats {
            hp: j.iter().find(|x| x.stat.name == "hp").map(|x| x.base_stat).unwrap_or(0),
            attack: j.iter().find(|x| x.stat.name == "attack").map(|x| x.base_stat).unwrap_or(0),
            defense: j.iter().find(|x| x.stat.name == "defense").map(|x| x.base_stat).unwrap_or(0),
            sp_attack: j.iter().find(|x| x.stat.name == "special-attack").map(|x| x.base_stat).unwrap_or(0),
            sp_defense: j.iter().find(|x| x.stat.name == "special-defense").map(|x| x.base_stat).unwrap_or(0),
            speed: j.iter().find(|x| x.stat.name == "speed").map(|x| x.base_stat).unwrap_or(0),
        }
    }
}
