use std::{env, process};
use grep_rs::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:\n{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error: {}", e);
        process::exit(2);
    }
}
