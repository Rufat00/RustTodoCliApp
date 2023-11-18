use console::style;
use std::process;

pub mod commands;
pub mod helpers;
pub mod messages;

pub struct Config {
    pub command: String,
    pub data: Vec<String>,
    pub flags: Vec<char>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            println!("{}", messages::HELP);
            process::exit(1)
        }

        let command = args[1].clone();
        let mut data: Vec<String> = vec![];
        let mut flags: Vec<char> = vec![];

        for argument in &args[2..] {
            if argument.starts_with('-') && argument.len() == 2 {
                for flag_char in argument.chars().skip(1) {
                    flags.push(flag_char);
                }
            } else {
                data.push(argument.clone());
            }
        }

        return Ok(Config {
            command,
            data,
            flags,
        });
    }
}

pub trait MyResultExt<T, E> {
    fn handle_error(self) -> T;
}

impl<T, E> MyResultExt<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn handle_error(self) -> T {
        return match self {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{} {}", style("Error:").red(), err);
                process::exit(1)
            }
        };
    }
}

#[macro_export]
macro_rules! return_error {
    ($msg:expr) => {{
        eprintln!("{} {}", console::style("Error:").red(), $msg);
        std::process::exit(1)
    }};
}
