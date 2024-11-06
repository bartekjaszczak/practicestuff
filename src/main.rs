use std::{env, process};
use practicestuff::{Application, Config};

fn main() {
    let args: Vec<_> = env::args().collect();
    // let command = &args[0];

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Failed to parse args: {err}");
        process::exit(1);
    });
    if let Err(err) = Application::run(&config) {
        println!("Failed to run application: {err}");
        process::exit(2);
    }
}
