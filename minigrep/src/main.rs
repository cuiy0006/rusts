use std::{ process, env };
use minigrep::{ Config, run };

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(&config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

