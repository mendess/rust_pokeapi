use ::types::TypeName;
use ::parser::pokemon::PkTypeJson;

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

