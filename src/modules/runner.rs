use crate::modules::config::Config;
use std::{io::stdin, ops::Deref};

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


pub struct Runner {
    pub mode: InterpreterModes,
    reader: Box<dyn Fn(& mut String)>
}

impl Runner {

    pub fn new (config: Config, reader: Option<Box<dyn Fn(& mut String)>>) -> Self {
        let has_no_path =  "".eq(&config.path);
        if has_no_path {
            Runner {
                mode: InterpreterModes::REPL,
                reader: match reader {
                    Some(function) => function,
                    None => Box::new(read_input),
                }
            }
        }  else { 
            Runner {
                mode: InterpreterModes::FILE,
                reader: Box::from(read_input),
            }
        }
    }

    pub fn startup(&self) {
        match self.mode {
            InterpreterModes::REPL => self.run_repl(),
            InterpreterModes::FILE => todo!(),
        }
    }


    fn run_file() {
        todo!()
    }
    fn run_repl(&self) {
        loop {
            print!("> ");
            let mut statement = String::new();
            self.reader.deref()(&mut statement);
            
            if statement == "" {
                break;
            }
            self.run(statement);

        }
    }

    fn run (&self, statement: String) {
        println!("{}", statement);
    }

    
    
}