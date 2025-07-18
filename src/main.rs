use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing problem {err}");
        process::exit(1)
    });

    if let Err(err) = run(config) {
        println!("{err}");
        process::exit(1)
    };
}
