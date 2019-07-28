use std::env;
use std::process;

mod lib;
use self::lib::{Options, run};

fn main() {
    let options = Options::new(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(options) {
        eprintln!("Error: {}", e);
        process::exit(1);
    };
}
