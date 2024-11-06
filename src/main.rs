use std::env;
use practicestuff::{Application, Config};

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = Config::parse_args(&args);
    Application::run(&config);
}
