use clap::Parser;
use std::process::exit;
use veil::{run, Cli};

fn main() {
    let cli = Cli::parse();
    match run(cli) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err.to_string());
            exit(1);
        }
    }
}
