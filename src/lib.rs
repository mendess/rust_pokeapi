extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod pokemon;
mod types;
mod abilities;
mod dex_error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn fetch_pokemon_bulbasaur_name() {
        match super::pokemon::Pokemon::fetch("bulbasaur".to_string()) {
            Ok(p) => super::assert_bulbasaur(p),
            Err(e) => println!("{}", e),
        }
    }

    fn fetch_pokemon_bulbasaur_number() {
        match super::pokemon::Pokemon::fetch("1".to_string()) {
            Ok(p) => super::assert_bulbasaur(p),
            Err(e) => println!("{}", e),
        }
    }
}

fn assert_bulbasaur(pk: pokemon::Pokemon) {
    assert_eq!(pk.id, 1);
    assert_eq!(pk.name, "bulbasaur");
}
