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
