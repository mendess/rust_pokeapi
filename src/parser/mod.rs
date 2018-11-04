pub mod pokemon;
pub mod abilities;
pub mod types;

#[derive(Deserialize)]
pub struct NamedAPIResource{
    pub name: String,
    pub url: String,
}
