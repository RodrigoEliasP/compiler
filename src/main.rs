mod modules;
use std::process::exit;

use modules::config;

fn main() {
    let configuration = config::Config::new(std::env::args());

    match configuration {
        Ok(config) => {
            println!("Path to parse {}", if "".eq(&config.path) {"repl".to_string()}  else { config.path.clone() }) ;
        },
        Err(err) => {
            println!("Error:");
            config::Config::show_help();
            exit(err as i32);
        },
    }
}
