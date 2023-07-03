pub fn builtin_echo(args: Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}
