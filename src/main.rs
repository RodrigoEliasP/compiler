mod modules;
use std::process::exit;

use modules::config;

fn main() {
    let configuration = config::Config::new(std::env::args());
    if configuration.is_err() {
        println!("Error:");
        config::Config::show_help();
        exit(configuration.err().unwrap() as i32);
    }
}
