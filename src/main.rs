extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod parser;
pub mod pokemon;
pub mod types;
pub mod abilities;
pub mod dex_error;

fn main() {

    match pokemon::Pokemon::fetch(1) {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("{}", e),
    }
}
