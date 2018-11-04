extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod parser;
pub mod pokemon;
pub mod types;
pub mod abilities;
pub mod dex_error;

#[cfg(test)]
mod tests {

    #[test]
    fn fetch_pokemon_bulbasaur_name() {
        match super::pokemon::Pokemon::fetch("bulbasaur".to_string()) {
            Ok(p) => assert_bulbasaur(p),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn fetch_pokemon_bulbasaur_number() {
        match super::pokemon::Pokemon::fetch(1) {
            Ok(p) => assert_bulbasaur(p),
            Err(e) => panic!("{}", e),
        }
    }

    fn assert_bulbasaur(pk: super::pokemon::Pokemon) {
        assert_eq!(pk.number(), 1);
        assert_eq!(pk.name(), "bulbasaur");
    }
}

