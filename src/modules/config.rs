use std::fmt::Debug;


pub enum ConfigErrors {
    ImproperUsage = 64
}

impl Debug for ConfigErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ImproperUsage => write!(f, "ImproperUsage"),
        }
    }
}

impl Into<i32> for ConfigErrors {
    fn into(self) -> i32 {
        self as i32
    }
}

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn show_help() {
        println!("
            Usage rlox [script]
        ");
    }

    fn extract_args (args: impl Iterator<Item=String>) -> Vec<String> {
        let mut args: Vec<String> = args.collect();
        args.drain(1..).collect()
    }

    fn validate_args (args: &Vec<String>) -> Result<(), ConfigErrors> {
        if args.len() > 1 {
            Err(ConfigErrors::ImproperUsage)
        } else {
            Ok(())
        }
    }

    fn build(args: Vec<String>) -> Self {
        if args.len() == 0 {
            Self {
                path: "".to_string(),
            }
        }
        else {
            Self {
                path: args[0].clone(),
            }
        }
    }

    pub fn new (args: impl Iterator<Item=String>) -> Result<Self, ConfigErrors> {
        let args_vec = Self::extract_args(args);
        let result = Self::validate_args(&args_vec);
        let config = Self::build(args_vec);
        
        match result {
            Ok(_) => Ok(config),
            Err(v) => Err(v),
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    const PATH_TO_COMPILE: &str = "path/to/compile.lox";
    #[test]
    fn should_parse_correctly() {
        let fake_args = ["ignored/program/path", PATH_TO_COMPILE];
        let config = Config::new(fake_args.into_iter().map(|x| x.to_string()));
        let config_path = &config.as_ref().unwrap().path;
        assert_eq!(config_path, PATH_TO_COMPILE);
        assert!(config.is_ok())
    }
    #[test]
    fn should_allow_repl_mode() {
        let fake_args = ["ignored/program/path"];
        let config = Config::new(fake_args.into_iter().map(|x| x.to_string()));
        let config_path = &config.as_ref().unwrap().path;
        assert_eq!(config_path, "");
        assert!(config.is_ok())
    }
    #[test]
    fn should_return_error_if_more_than_one_argument(){
        let fake_args = ["ignored/program/path", "path/to/compile.lox", "extra_argument"];
        let config = Config::new(fake_args.into_iter().map(|x| x.to_string()));
        assert!( config.is_err());
    }
}
