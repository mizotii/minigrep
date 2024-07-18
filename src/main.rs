use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // iterates over command line arguments and puts them into a vector
    // panics if any argument includes invalid unicode
    let args: Vec<String> = env::args().collect(); // we usually want to annotate collect
    
    // reads args into config
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    // reads the file
    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}