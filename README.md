# rust_pokeapi
A [poke_api](https://pokeapi.co) wrapper in rust


## TODO
- Default to lattest generation
- Provide a way to pick a generation


## Ideas
When you fetch an object that has other objects "hanging", provide a method to fetch the full hanging object, for example, if I fetch a Pokemon it comes with it's abilites in a simplified fashion, these abilities should provide a method to get the full ability object
