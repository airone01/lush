use std::str::FromStr;

use crate::builtin::dir::{builtin_cd, builtin_pwd};
use crate::builtin::echo::builtin_echo;
use crate::builtin::who::builtin_whoami;
use crate::command::Command;

pub enum BuiltIn {
    Cd,
    Echo,
    History,
    Pwd,
    WhoAmI,
}

impl FromStr for BuiltIn {
    type Err = ();
    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        match entry {
            "cd" => Ok(BuiltIn::Cd),
            "echo" => Ok(BuiltIn::Echo),
            "history" => Ok(BuiltIn::History),
            "pwd" => Ok(BuiltIn::Pwd),
            "whoami" => Ok(BuiltIn::WhoAmI),
            _ => Err(()),
        }
    }
}

pub fn process_command(command: Command) -> i32 {
    match BuiltIn::from_str(&command.keyword) {
        Ok(BuiltIn::Cd) => builtin_cd(command.args),
        Ok(BuiltIn::Echo) => builtin_echo(command.args),
        Ok(BuiltIn::History) => {
            println!("History");
            0
        }
        Ok(BuiltIn::Pwd) => builtin_pwd(),
        Ok(BuiltIn::WhoAmI) => builtin_whoami(),
        Err(_) => {
            println!("Command not found: {}", command.keyword);
            1
        }
    }
}
