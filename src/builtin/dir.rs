use clap::{Arg, ArgAction, Command};
use home::home_dir;
use path_absolutize::*;
use std::env::set_current_dir;
use std::path::{Path, PathBuf};

pub fn builtin_cd(raw_args: Vec<String>) -> i32 {
    let mut clap_args = vec!["cd".to_string()];
    clap_args.extend(raw_args.clone());
    // note that this ignores the fact that the command could have another name rather than "cd" (with aliases, etc.)
    // too lazy to fix this right now

    let matches_result = Command::new("cd")
        .about("Lush built-in. Change the shell working directory.")
        .author("Lush team")
        .arg(
            Arg::new("path")
                .index(1)
                .action(ArgAction::Set)
                .default_value("~")
                .value_name("PATH")
                .help("the path to change to"),
        )
        .try_get_matches_from(clap_args);
    let matches = match matches_result {
        Ok(matches) => matches,
        Err(error) => {
            println!("{}", error);
            return 1;
        }
    };

    let input_path = match matches.get_one::<String>("path") {
        Some(path) => path.to_string(),
        None => {
            println!("Could not get path argument");
            return 1;
        }
    };

    let cwd = match get_cwd(false) {
        Ok(cwd) => cwd,
        Err(error) => {
            println!("Could not get current directory: {:?}", error);
            return 1;
        }
    };

    let asb_path = match resolve_path(input_path, cwd, home_dir, true) {
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

fn resolve_path(
    input_path: String,
    cwd: String,
    get_home: fn() -> Option<PathBuf>,
    check_dir: bool,
) -> Result<String, i32> {
    let trimmed_input_path = input_path.trim().to_string();
    let computed_path: String;
    let is_empty = trimmed_input_path.len() == 0;
    if trimmed_input_path.starts_with("~") || is_empty {
        if let Some(home) = get_home() {
            if is_empty {
                computed_path = home.to_str().unwrap().to_string();
            } else if trimmed_input_path.starts_with("~/") {
                computed_path =
                    trimmed_input_path.replace("~/", &format!("{}/", home.to_str().unwrap()));
            } else if trimmed_input_path == "~" {
                computed_path = trimmed_input_path.replace("~", home.to_str().unwrap());
            } else {
                computed_path = trimmed_input_path.clone();
            }
        } else {
            println!("The home directory doesn't exist.");
            return Err(2);
        }
    } else {
        computed_path = trimmed_input_path.clone();
    }

    if let Ok(asb_path) = Path::new(&computed_path).absolutize_from(cwd) {
        if !check_dir || asb_path.exists() {
            if !check_dir || asb_path.is_dir() {
                return Ok(asb_path.to_string_lossy().to_string());
            } else {
                println!("Not a directory: {:?}", &computed_path);
                return Err(20);
            }
        }
        println!("No such file or directory: {:?}", &computed_path);
        return Err(2);
    }
    println!("Invalid path: {:?}", &computed_path);
    Err(22)
}

pub fn builtin_pwd(raw_args: Vec<String>) -> i32 {
    let mut clap_args = vec!["pwd".to_string()];
    clap_args.extend(raw_args.clone());
    // note that this ignores the fact that the command could have another name rather than "pwd" (with aliases, etc.)
    // too lazy to fix this right now

    let matches_result = Command::new("pwd")
        .about("Lush built-in. Print the name of the current working directory.")
        .author("Lush team")
        .try_get_matches_from(clap_args);

    match matches_result {
        Ok(matches) => matches,
        Err(error) => {
            println!("{}", error);
            return 1;
        }
    };

    if let Ok(pwd_dir) = get_cwd(false) {
        println!("{}", pwd_dir);
        0
    } else {
        println!("Could not get current directory");
        1
    }
}

pub fn get_cwd(shorten_when_possible: bool) -> Result<String, std::io::Error> {
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

// List of unit tests for the builtin_dir module
// - empty
// - .
// - ./
// - ./dir
// - ./dir/
// - ./.
// - ././
// - ./..
// - ./../
// - ..
// - ../
// - ../dir
// - ../dir/
// - ../.
// - .././
// - ../..
// - ../../
// - ~
// - ~/
// - ~/dir
// - ~/dir/
// - ~/.
// - ~/./
// - ~/..
// - ~/../
// - /
// - /dir
// - /dir/
// - /.
// - /./
// - /..
// - /../
// - /~
// - error: can't find home directory
// - error: `...`
// - error: dir doesn't exist
// - error: dir isn't a directory
// - temp path
// - error: too many arguments
// - " ~ "
// - ~ dir
// - dir
// - dir/~
// - cd to temp path
#[cfg(test)]
mod unittest_dir {
    use test_dir::{DirBuilder, FileType, TestDir};

    use super::*; // get all the functions from the parent file

    fn mock_get_home() -> Option<PathBuf> {
        Some(PathBuf::from("/home/user"))
    }

    fn resolve_path_trimmed(input: &str) -> Result<String, i32> {
        resolve_path(
            input.to_string(),
            "/home/user".to_string(),
            mock_get_home,
            false,
        )
    }

    #[test]
    fn empty() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed(""));
    }

    #[test]
    fn dot() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("."));
    }

    #[test]
    fn dot_slash() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("./"));
    }

    #[test]
    fn dot_slash_dir() {
        assert_eq!(
            Ok("/home/user/dir".to_string()),
            resolve_path_trimmed("./dir")
        );
    }

    #[test]
    fn dot_slash_dir_slash() {
        assert_eq!(
            Ok("/home/user/dir".to_string()),
            resolve_path_trimmed("./dir/")
        );
    }

    #[test]
    fn dot_slash_dot() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("./."));
    }

    #[test]
    fn dot_slash_dot_slash() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("././"));
    }

    #[test]
    fn dot_slash_dot_dot() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed("./.."));
    }

    #[test]
    fn dot_slash_dot_dot_slash() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed("./../"));
    }

    #[test]
    fn dot_dot() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed(".."));
    }

    #[test]
    fn dot_dot_slash() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed("../"));
    }

    #[test]
    fn dot_dot_slash_dir() {
        assert_eq!(Ok("/home/dir".to_string()), resolve_path_trimmed("../dir"));
    }

    #[test]
    fn dot_dot_slash_dir_slash() {
        assert_eq!(Ok("/home/dir".to_string()), resolve_path_trimmed("../dir/"));
    }

    #[test]
    fn dot_dot_slash_dot() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed("../."));
    }

    #[test]
    fn dot_dot_slash_dot_slash() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed(".././"));
    }

    #[test]
    fn dot_dot_slash_dot_dot() {
        assert_eq!(Ok("/".to_string()), resolve_path_trimmed("../.."));
    }

    #[test]
    fn dot_dot_slash_dot_dot_slash() {
        assert_eq!(Ok("/".to_string()), resolve_path_trimmed("../../"));
    }

    #[test]
    fn tilde() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("~"));
    }

    #[test]
    fn tilde_slash() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("~/"));
    }

    #[test]
    fn tilde_slash_dir() {
        assert_eq!(
            Ok("/home/user/dir".to_string()),
            resolve_path_trimmed("~/dir")
        );
    }

    #[test]
    fn tilde_slash_dir_slash() {
        assert_eq!(
            Ok("/home/user/dir".to_string()),
            resolve_path_trimmed("~/dir/")
        );
    }

    #[test]
    fn tilde_slash_dot() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("~/."));
    }

    #[test]
    fn tilde_slash_dot_slash() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed("~/./"));
    }

    #[test]
    fn tilde_slash_dot_dot() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed("~/.."));
    }

    #[test]
    fn tilde_slash_dot_dot_slash() {
        assert_eq!(Ok("/home".to_string()), resolve_path_trimmed("~/../"));
    }

    #[test]
    fn slash() {
        assert_eq!(Ok("/".to_string()), resolve_path_trimmed("/"));
    }

    #[test]
    fn slash_dir() {
        assert_eq!(Ok("/dir".to_string()), resolve_path_trimmed("/dir"));
    }

    #[test]
    fn slash_dir_slash() {
        assert_eq!(Ok("/dir".to_string()), resolve_path_trimmed("/dir/"));
    }

    #[test]
    fn slash_dot() {
        assert_eq!(Ok("/".to_string()), resolve_path_trimmed("/."));
    }

    #[test]
    fn slash_dot_slash() {
        assert_eq!(Ok("/".to_string()), resolve_path_trimmed("/./"));
    }

    #[test]
    fn slash_dot_dot() {
        assert_eq!(Ok("/".to_string()), resolve_path_trimmed("/.."));
    }

    #[test]
    fn slash_dot_dot_slash() {
        assert_eq!(Ok("/".to_string()), resolve_path_trimmed("/../"));
    }

    #[test]
    fn slash_tilde() {
        assert_eq!(Ok("/~".to_string()), resolve_path_trimmed("/~"));
    }

    #[test]
    fn error_enoent_no_home() {
        assert_eq!(
            Err(2),
            resolve_path("~".to_string(), "/home/user".to_string(), || None, false)
        );
    }

    #[test]
    fn error_enoent() {
        assert_eq!(
            Err(2),
            resolve_path(
                rand::Rng::sample_iter(rand::thread_rng(), &rand::distributions::Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect(),
                "/".to_string(),
                || None,
                true
            )
        );
    }

    #[test]
    fn error_not_dir() {
        let cwd = get_cwd(false).expect("should find cwd in test environment");
        let temp = TestDir::temp().create("test/file", FileType::EmptyFile);
        let path = temp.path("test/file");

        assert_eq!(
            Err(20),
            resolve_path(path.to_string_lossy().to_string(), cwd, || None, true)
        );
    }

    #[test]
    fn temp_path() {
        let cwd = get_cwd(false).expect("should find cwd in test environment");
        let temp = TestDir::temp().create("test/dir", FileType::Dir);
        let path = temp.path("test/dir");

        assert_eq!(
            Ok(path.to_string_lossy().to_string()),
            resolve_path(path.to_string_lossy().to_string(), cwd, || None, false)
        );
    }

    // #[test]
    // fn error_too_many_args() {
    //     assert_eq!(
    //         7,
    //         builtin_cd(vec![
    //             "one".to_string(),
    //             "two".to_string(),
    //             "three".to_string()
    //         ])
    //     );
    // }

    #[test]
    fn space_tilde_space() {
        assert_eq!(Ok("/home/user".to_string()), resolve_path_trimmed(" ~ "));
    }

    #[test]
    fn tilde_dir() {
        assert_eq!(
            Ok("/home/user/~ dir".to_string()),
            resolve_path_trimmed("~ dir")
        );
    }

    #[test]
    fn dir() {
        assert_eq!(
            Ok("/home/user/dir".to_string()),
            resolve_path_trimmed("dir")
        );
    }

    #[test]
    fn dir_slash_tilde() {
        assert_eq!(
            Ok("/home/user/dir/~".to_string()),
            resolve_path_trimmed("dir/~")
        );
    }

    // #[test]
    // #[ignore = "breaks `error_not_dir` and `temp_path`"]
    // fn cd_to_temp() {
    //     let temp = TestDir::temp().create("test/dir", FileType::Dir);
    //     let path = temp.path("test/dir");

    //     assert_eq!(0, builtin_cd(vec![path.to_string_lossy().to_string()]));
    // }
}
