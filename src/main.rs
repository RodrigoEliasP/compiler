mod modules;
use std::process::exit;

use modules::config::Config;
use modules::runner::Runner;

fn main() {
    let configuration = Config::new(std::env::args());

    match configuration {
        Ok(config) => {
            let runner = Runner::new(config, None);
            runner.startup();
        },
        Err(err) => {
            println!("Error:");
            Config::show_help();
            exit(err as i32);
        },
    }
}
