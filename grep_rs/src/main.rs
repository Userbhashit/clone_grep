use clap::Parser;
use grep_rs;

fn main() {
    let config = grep_rs::Config::parse();
    let _ = grep_rs::run(config);
}
