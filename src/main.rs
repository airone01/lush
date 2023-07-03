use std::io::{self, Write};

mod builtin;
mod command;
mod tokenize;

use crate::builtin::builtin::process_command;
use crate::tokenize::tokenize_command;

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
