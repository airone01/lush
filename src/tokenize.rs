use crate::command::Command;

pub fn tokenize_command(command: String) -> Command {
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
