use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    asteroids_lib::run()
}
