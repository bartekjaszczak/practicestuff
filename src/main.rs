use practicestuff::{Application, Config};
use std::{env, process};

fn main() {
    let args: Vec<_> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    Application::run(&config);
}
