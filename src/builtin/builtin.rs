use std::str::FromStr;

use crate::builtin::echo::builtin_echo;
use crate::command::Command;

pub enum BuiltIn {
    Echo,
    History,
    Cd,
    Pwd,
}

impl FromStr for BuiltIn {
    type Err = ();
    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        match entry {
            "echo" => Ok(BuiltIn::Echo),
            "history" => Ok(BuiltIn::History),
            "cd" => Ok(BuiltIn::Cd),
            "pwd" => Ok(BuiltIn::Pwd),
            _ => Err(()),
        }
    }
}

pub fn process_command(command: Command) -> i32 {
    match BuiltIn::from_str(&command.keyword) {
        Ok(BuiltIn::Echo) => builtin_echo(command.args),
        Ok(BuiltIn::History) => {
            println!("History");
            0
        }
        Ok(BuiltIn::Cd) => {
            println!("Cd");
            0
        }
        Ok(BuiltIn::Pwd) => {
            println!("Pwd");
            0
        }
        Err(_) => {
            println!("Command not found: {}", command.keyword);
            1
        }
    }
}
