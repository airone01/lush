use std::fmt::{self, Debug, Formatter};
use std::io::{self, Write};
use std::str::FromStr;

struct Command {
    keyword: String,
    args: Vec<String>,
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Command {{ keyword: {}, args: {:?} }}",
            self.keyword, self.args
        )
    }
}

fn tokenize_command(command: String) -> Command {
    let mut command_split: Vec<String> = command
        .split_whitespace()
        .map(|raw_split| raw_split.to_string())
        .collect();
    println!("DEBUG: Split input: {:?}", command_split);

    Command {
        keyword: command_split.remove(0),
        args: command_split,
    }
}

#[cfg(test)]
mod unittest_tokenize_command {
    use super::*;

    #[test]
    #[ignore]
    fn empty_command() {
        assert_eq!("", tokenize_command("".to_string()).keyword)
    }

    #[test]
    fn test_keyword() {
        assert_eq!("test", tokenize_command("test".to_string()).keyword)
    }

    #[test]
    fn no_arg() {
        assert_eq!(0, tokenize_command("test".to_string()).args.len())
    }

    #[test]
    fn one_arg() {
        assert_eq!(1, tokenize_command("test one".to_string()).args.len())
    }

    #[test]
    fn multi_args() {
        assert_eq!(
            3,
            tokenize_command("test one two three".to_string())
                .args
                .len()
        )
    }

    #[test]
    #[ignore]
    fn quotes() {
        assert_eq!(
            2,
            tokenize_command("test \"one two\" three".to_string())
                .args
                .len()
        )
    }
}

enum BuiltIn {
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

fn builtin_echo(args: Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

fn process_command(command: Command) -> i32 {
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

fn main() {
    let promt_text = "%";

    loop {
        print!("{} ", promt_text);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input_split = tokenize_command(input);
        println!("DEBUG: {:?}", input_split);

        process_command(input_split);
    }
}
