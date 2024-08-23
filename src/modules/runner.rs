use crate::modules::config::Config;
use std::io::stdin;
use std::fs::read_to_string;

#[derive(PartialEq, Eq)]
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
        println!("{}", script);
    }

    
    
}