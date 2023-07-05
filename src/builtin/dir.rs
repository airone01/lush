use home::home_dir;
use path_absolutize::*;
use std::env::set_current_dir;
use std::path::{Path, PathBuf};

pub fn builtin_cd(args: Vec<String>) -> i32 {
    if args.len() > 1 {
        println!("Too many arguments");
        return 7;
    }
    let input_path = args[0].clone();

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
    let computed_path: String;
    if input_path.starts_with("~") || input_path.len() == 0 {
        if let Some(home) = get_home() {
            computed_path = input_path.replace("~", home.to_str().unwrap());
        } else {
            println!("The home directory doesn't exist.");
            return Err(2);
        }
    } else {
        computed_path = input_path.clone();
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

pub fn builtin_pwd() -> i32 {
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
// - random path
#[cfg(test)]
mod unittest_dir {
    use test_dir::{DirBuilder, FileType, TestDir};

    use super::*; // get all the functions from the parent file

    #[ignore = "mock function"]
    fn mock_get_home() -> Option<PathBuf> {
        Some(PathBuf::from("/home/user"))
    }

    #[ignore = "mock function"]
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
    fn random_path() {
        let cwd = get_cwd(false).expect("should find cwd in test environment");
        let temp = TestDir::temp().create("test/dir", FileType::Dir);
        let path = temp.path("test/dir");

        assert_eq!(
            Ok(path.to_string_lossy().to_string()),
            resolve_path(path.to_string_lossy().to_string(), cwd, || None, false)
        );
    }
}
