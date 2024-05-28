use clap::{Arg, ArgAction, Command};

pub fn builtin_echo(raw_args: Vec<String>) -> i32 {
    let mut clap_args = vec!["echo".to_string()];
    clap_args.extend(raw_args.clone());
    // note that this ignores the fact that the command could have another name rather than "echo" (with aliases, etc.)
    // too lazy to fix this right now

    let matches_result = Command::new("echo")
        .about("Lush built-in. Write sentence to the standard output.")
        .author("Lush team")
        .arg(
            Arg::new("sentence")
                .index(1)
                .action(ArgAction::Set)
                .default_value("")
                .value_name("SENTENCE")
                .help("what to echo"),
        )
        .try_get_matches_from(clap_args);
    let matches = match matches_result {
        Ok(matches) => matches,
        Err(error) => {
            println!("{}", error);
            return 1;
        }
    };

    let input_sentence = match matches.get_one::<String>("path") {
        Some(path) => path.to_string(),
        None => {
            println!("Could not get path argument");
            return 1;
        }
    };

    println!("{}", input_sentence);
    0
}
