pub fn builtin_whoami(args: Vec<String>) -> i32 {
    println!("{}", get_user_username());
    0
}

pub fn get_user_username() -> String {
    whoami::username()
}

pub fn get_user_hostname() -> String {
    whoami::hostname()
}
