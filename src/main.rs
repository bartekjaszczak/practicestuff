use std::{env, process};
use practicestuff::{Application, Config};

fn main() {
    let args: Vec<_> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = Application::run(&config) {
        eprintln!("{err}");
        process::exit(2);
    }
}
