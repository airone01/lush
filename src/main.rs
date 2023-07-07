use std::io::{stdout, Stdout, Write};
use std::time::Duration;

use builtin::builtin::process_command;
use builtin::dir::get_cwd;
use builtin::who::{get_user_hostname, get_user_username};
use crossterm::event::KeyEventKind::Release;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{self, ClearType, ScrollUp};
use tokenize::tokenize_command;

mod builtin;
mod command;
mod tokenize;

fn setup_term() -> Stdout {
    let stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout
}

fn reset_term(stdout: &mut Stdout) {
    // terminal::disable_raw_mode().unwrap();
    queue!(stdout, terminal::Clear(ClearType::CurrentLine)).unwrap();
    print!("\r");
    stdout.flush().unwrap();
}

fn get_prompt() -> String {
    format!(
        "{}@{}:{}",
        get_user_username(),
        get_user_hostname(),
        get_cwd(true).unwrap_or("???".to_string())
    )
}

fn next_term(stdout: &mut Stdout, scroll_up: bool, write_to_eol: Option<&str>) -> String {
    match (write_to_eol, scroll_up) {
        (Some(write_to_eol), true) => {
            queue!(stdout, Print(write_to_eol), ScrollUp(1), Print("\r")).unwrap();
        }
        (Some(write_to_eol), false) => {
            queue!(stdout, Print(write_to_eol), Print("\r")).unwrap();
        }
        (None, true) => {
            queue!(stdout, ScrollUp(1), Print("\r")).unwrap();
        }
        (None, false) => {
            queue!(stdout, Print("\r")).unwrap();
        }
    }
    print_prompt(stdout, get_prompt(), "".to_string());
    "".to_string()
}

fn print_prompt(stdout: &mut Stdout, prompt: String, buff: String) {
    queue!(stdout, Print(format!("{} {}", prompt, buff))).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    term();
}

fn term() {
    let mut buff = String::new();
    let mut stdout = setup_term();

    reset_term(&mut stdout);
    print_prompt(&mut stdout, get_prompt(), buff.clone());

    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                Event::Key(KeyEvent { kind: Release, .. }) => {
                    continue;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    if buff.len() > 0 {
                        let command = tokenize_command(buff.clone());
                        queue!(stdout, ScrollUp(1), Print("\r")).unwrap();
                        process_command(command);
                        buff = next_term(&mut stdout, false, None);
                    } else {
                        buff = next_term(&mut stdout, true, None);
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    buff = next_term(&mut stdout, true, Some("^C"));
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    if buff.len() > 0 {
                        buff.pop();
                        queue!(stdout, Print("\x08 \x08")).unwrap();
                        stdout.flush().unwrap();
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(char),
                    ..
                }) => {
                    buff.push(char);
                    queue!(stdout, Print(char)).unwrap();
                    stdout.flush().unwrap();
                }
                _ => {
                    continue;
                }
            }
        };
    }
}
