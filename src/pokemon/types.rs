use ::types::TypeName;

#[derive(Debug)]
pub struct PkType {
    pub slot: u16,
    pub name: TypeName,
}

impl<'a> From<&'a PkTypeJson> for PkType {
    fn from(tp: &PkTypeJson) -> Self {
        PkType{
            slot: tp.slot,
            name: self::TypeName::from(tp.type_name.name.clone())
        }
    }
}

#[derive(Deserialize)]
pub struct PkTypeJson {
    slot: u16,
    #[serde(rename = "type")]
    type_name: PkTpJson,
}

#[derive(Deserialize)]
struct PkTpJson {
    url: String,
    name: String,
}
