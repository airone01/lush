use home::home_dir;
use path_absolutize::*;
use std::env::set_current_dir;
use std::path::Path;

pub fn builtin_cd(args: Vec<String>) -> i32 {
    let asb_path = match resolve_path(args, false) {
        Ok(path) => path,
        Err(error) => return error,
    };

    match set_current_dir(&asb_path) {
        Ok(_) => 0,
        Err(error) => {
            println!("{}: {:?}", error, asb_path);
            error
                .raw_os_error() // Get OS error number
                .unwrap_or(1) //By default, 1 (EPERM - Operation not permitted)
        }
    }
}

fn resolve_path(args: Vec<String>, dont_check_dir: bool) -> Result<String, i32> {
    let path: String;
    if args[0].starts_with("~") || args.len() == 0 {
        if let Some(home) = home_dir() {
            if args.len() == 0 {
                path = home.to_string_lossy().to_string();
            } else {
                path = home.join(&args[0][2..]).to_string_lossy().to_string();
            }
        } else {
            println!("The home directory doesn't exist.");
            return Err(2);
        }
    } else {
        path = args[0].clone();
    }

    if let Ok(asb_path) = Path::new(&path).absolutize() {
        if dont_check_dir || asb_path.is_dir() {
            Ok(asb_path.to_string_lossy().to_string())
        } else {
            println!("Not a directory: {:?}", &path);
            Err(20)
        }
    } else {
        println!("Invalid path: {:?}", &path);
        Err(22)
    }
}

pub fn builtin_pwd() -> i32 {
    if let Ok(pwd_dir) = get_current_dir_string(false) {
        println!("{}", pwd_dir);
        0
    } else {
        println!("Could not get current directory");
        1
    }
}

pub fn get_current_dir_string(shorten_when_possible: bool) -> Result<String, std::io::Error> {
    match std::env::current_dir() {
        Ok(path_buf) => {
            if shorten_when_possible && path_buf == home_dir().unwrap() {
                Ok("~".to_string())
            } else {
                Ok(path_buf.display().to_string())
            }
        }
        Err(error) => Err(error),
    }
}
