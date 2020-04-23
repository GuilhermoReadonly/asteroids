# Another Asteroid Game
This is just a pet project whose main goal is for me to learn the rust language.  
And also what is more fun than playing a game ? Making a game !

So nothing fancy, just an old fashioned asteroids game.

## Roadmap (more or less by priorities):
* [ ] Improve hitbox
  * [x] Part one use simple rectangles
  * [ ] Part two use Gilbert–Johnson–Keerthi distance algorithm: https://en.wikipedia.org/wiki/Gilbert%E2%80%93Johnson%E2%80%93Keerthi_distance_algorithm
* [x] Improve physics
* [ ] Refactor the codebase (it is a mess)
* [ ] Improve spawn (check if space is clear before spawning)
* [ ] Add screens:
  * [ ] Game over
  * [ ] Settings:
    * [ ] Keyboard
    * [ ] Sounds on/off
  * [ ] Score boards
  * [ ] Credits
* [ ] Sounds effects (Pewpew)
* [ ] Animations:
  * [ ] Bullets hits
  * [ ] Rocks
* [ ] Differents kind of guns
* [ ] Add saucers
* [ ] Increase world size and stay centred on the player ship
* [ ] Multiplayer mode
* [ ] WASM version playable in the browser


# Builds and stuff

## Stuff to have installed before build

### Rust:
https://www.rust-lang.org/tools/install


## Build & Run

```
cargo run --release
```

## Build

```
cargo build --release
```

The executable file is generated in the ```./target/release/``` folder.


## Cross compillation target build

### Windows
Prepare environment:
```
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

Build:

```
cargo build --release --target=x86_64-pc-windows-gnu
```

### Mac (work in progress)
Prepare environment:
```
rustup target add x86_64-apple-darwin
```

Build:

```
cargo build --release --target=x86_64-apple-darwin
```

