pub fn get_user_username() -> String {
    whoami::username()
}

pub fn get_user_hostname() -> String {
    whoami::hostname()
}
