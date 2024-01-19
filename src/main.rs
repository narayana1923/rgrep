// To read command line arguments
use std::env::args;
use std::process;

// Our library things
use rgrep::{run, Config};

fn main() {
    let cmd_args: Vec<String> = args().collect();
    let config = Config::build(&cmd_args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
