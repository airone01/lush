use home::home_dir;
use std::env::set_current_dir;
use std::path::PathBuf;

pub fn builtin_cd(args: Vec<String>) -> i32 {
    let path: &str;
    if args.len() > 0 {
        path = &args[0];
    } else {
        path = "";
    }

    set_current_dir(get_meant_dir(path.to_string())).unwrap();
    0
}

pub fn builtin_pwd() -> i32 {
    println!("{}", get_current_dir_string(false));
    0
}

fn get_meant_dir(dir: String) -> PathBuf {
    let mut mut_entry = dir.clone();
    let mut path_buf = PathBuf::new();

    if mut_entry.starts_with("~/") {
        path_buf.push(home_dir().unwrap());
        mut_entry.remove(0);
        mut_entry.remove(0);
        path_buf.push(mut_entry);
    } else if mut_entry == "~" || mut_entry == "" {
        path_buf.push(home_dir().unwrap());
    } else {
        path_buf.push(mut_entry);
    }

    path_buf
}

pub fn get_current_dir_string(shorten_when_possible: bool) -> String {
    let path_buf: PathBuf = std::env::current_dir().unwrap();

    if shorten_when_possible && path_buf == home_dir().unwrap() {
        "~".to_string()
    } else {
        path_buf.display().to_string()
    }
}
