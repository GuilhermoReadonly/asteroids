# Another Asteroid Game
This is just a pet project whose main goal is for me to learn the rust language and do a bit of game development.
So nothing fancy, just an old fashioned asteroid game.

This is a Work In Progress.

## Stuff to have installed before build

### Rust
https://www.rust-lang.org/tools/install


## Build & Run

```
cargo run --release
```

## Cross target build
Install the desired target (here windows with gnu tools):

```
rustup target add x86_64-pc-windows-gnu
```

Build with the desired installed target:

```
cargo build --release --target=x86_64-pc-windows-gnu
```

The executable file is generated in the ```./target/release/``` folder.
