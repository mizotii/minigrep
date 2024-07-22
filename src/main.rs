use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // reads args into config
    let config = Config::build(env::args()).unwrap_or_else(|err| { // args() returns an iterator
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    // reads the file
    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}