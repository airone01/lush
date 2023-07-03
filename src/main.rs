use std::io::{self, Write};

mod builtin;
mod command;
mod tokenize;

use crate::builtin::builtin::process_command;
use crate::builtin::who::{get_user_hostname, get_user_username};
use crate::tokenize::tokenize_command;

fn main() {
    loop {
        print!(
            "{}@{}:{} ",
            get_user_username(),
            get_user_hostname(),
            std::env::current_dir().unwrap().display()
        );
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
