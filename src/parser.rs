pub mod pokemon;
pub mod abilities;
pub mod types;

#[derive(Debug,Deserialize)]
pub struct NamedAPIResource{
    pub name: String,
    pub url: String,
}
