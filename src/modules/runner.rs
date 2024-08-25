use super::config::Config;
use super::scanner::Scanner;
use std::io::stdin;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Debug)]
pub enum InterpreterModes{
    REPL,
    FILE,
}

fn read_input(buffer: &mut String) {
    let result = stdin().read_line(buffer);
    match result {
        Ok(_) => {  },
        Err(err) => {
            panic!("Error while reading from stdin {err}")
        },
    }
}


pub struct Runner<'a> {
    pub mode: InterpreterModes,
    config: Config,
    reader: Box<dyn for<'b> FnMut(&'b mut String) + 'a>
}

impl<'a> Runner<'a> {

    pub fn new (config: Config, reader: Option<Box<dyn for<'b> FnMut(&'b mut String) + 'a>>) -> Self {
        let has_no_path =  "".eq(&config.path);
        if has_no_path {
            Runner {
                mode: InterpreterModes::REPL,
                config,
                reader: match reader {
                    Some(function) => function,
                    None => Box::new(read_input),
                }
            }
        }  else { 
            Runner {
                mode: InterpreterModes::FILE,
                config,
                reader: Box::from(read_input),
            }
        }
    }

    pub fn startup(&mut self) {
        let config_path = &self.config.path.clone();
        match self.mode {
            InterpreterModes::REPL => self.run_repl(),
            InterpreterModes::FILE => self.run_file(config_path),
        }
    }


    fn run_file(&mut self, path: &String) {
        let contents = read_to_string(path);
        let script = contents.expect(&format!("Error while opening file in {path}").to_string());
        self.run(script)
    }
    fn run_repl(&mut self) {
        loop {
            print!("> ");
            let mut statement: String = String::new();
            (self.reader)(& mut statement);
            
            if statement == "" {
                break;
            }
            self.run(statement);

        }
    }

    fn run (&self, script: String) {
        let mut scanner = Scanner::new(script);
        let token_list = scanner.scan_tokens();

        println!("{:?}", token_list);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    #[test]
    fn should_bootstrap_in_repl_mode() {
        let call_times = Rc::new(RefCell::new(0));
        let call_times_clone = call_times.clone();
        let fake_reader = |buffer: &mut String| {
            *call_times_clone.borrow_mut() += 1;
            if *call_times_clone.borrow() < 2 {
                *buffer = "testing placeholder".to_string();
            } else {
                *buffer = "".to_string();   
            }
        };
        let fake_config = Config {
            path: "".to_string(),
        };
        let mut runner = Runner::new(fake_config, Some(Box::new(fake_reader)));
        runner.startup();
        assert_eq!(runner.mode, InterpreterModes::REPL);
        assert_eq!(*call_times.borrow(), 2);
    }
    #[test]
    fn should_read_file_correctly() {
        let fake_config = Config {
            path: "lox_scripts/file_reading_test.txt".to_string(),
        };
        let mut runner = Runner::new(fake_config, None);
        runner.startup();
        assert_eq!(runner.mode, InterpreterModes::FILE);
    }
}
