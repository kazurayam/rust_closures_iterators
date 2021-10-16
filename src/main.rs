use std::env;
use std::process;

use rust_closures_iterators::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("   query: {}", config.query);
    println!("filename: {}", config.filename);

    if let Err(e) = rust_closures_iterators::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
