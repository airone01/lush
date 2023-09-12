use std::io::{stdout, Stdout, Write};
use std::time::Duration;

use builtin::builtin::process_command;
use builtin::dir::get_cwd;
use builtin::who::{get_user_hostname, get_user_username};
use crossterm::cursor::{self, MoveLeft, MoveRight, MoveTo, MoveToNextLine};
use crossterm::event::KeyEventKind::Release;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self, ClearType, EnableLineWrap, ScrollUp};
use crossterm::{execute, queue};
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

fn print_nextln(
    stdout: &mut Stdout,
    newline: bool,
    write_to_eol: Option<&str>,
    print_text: fn(stdout: &mut Stdout) -> (),
) {
    let (_, y) = cursor::position().unwrap();
    let (_, ly) = terminal::size().unwrap();

    match (write_to_eol, newline, y >= ly - 1) {
        (Some(write_to_eol), true, false) => {
            queue!(stdout, Print(write_to_eol), MoveToNextLine(1), Print("\r")).unwrap();
        }
        (Some(write_to_eol), true, true) => {
            queue!(
                stdout,
                Print(write_to_eol),
                ScrollUp(1),
                MoveToNextLine(1),
                Print("\r")
            )
            .unwrap();
        }
        (Some(write_to_eol), false, _) => {
            queue!(stdout, Print(write_to_eol), Print("\r")).unwrap();
        }
        (None, true, true) => {
            queue!(stdout, ScrollUp(1), MoveToNextLine(1), Print("\r")).unwrap();
        }
        (None, true, false) => {
            queue!(stdout, MoveToNextLine(1), Print("\r")).unwrap();
        }
        (None, false, _) => {
            queue!(stdout, Print("\r")).unwrap();
        }
    }
    print_text(stdout);
}

fn next_term(stdout: &mut Stdout, newline: bool, write_to_eol: Option<&str>) -> (String, usize) {
    fn print_text(stdout: &mut Stdout) {
        print_prompt(stdout, get_prompt(), "".to_string());
        ()
    }

    print_nextln(stdout, newline, write_to_eol, print_text);
    ("".to_string(), 0)
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
    let mut position_relative_to_end: usize = 0;

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
                    let newline: bool;
                    if buff.len() > 0 {
                        let command = tokenize_command(buff.clone());
                        queue!(stdout, MoveToNextLine(1), Print("\r")).unwrap();
                        process_command(command);
                        newline = false;
                    } else {
                        newline = true;
                    }
                    (buff, position_relative_to_end) = next_term(&mut stdout, newline, None);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    (buff, position_relative_to_end) = next_term(&mut stdout, true, Some("^C"));
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    queue!(
                        stdout,
                        MoveTo(0, 0),
                        terminal::Clear(ClearType::All),
                        EnableLineWrap
                    )
                    .unwrap();
                    (buff, position_relative_to_end) = next_term(&mut stdout, false, None);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => {
                    if buff.len() > 0 && position_relative_to_end < buff.len() {
                        position_relative_to_end += 1;

                        execute!(stdout, MoveLeft(1)).unwrap();
                        stdout.flush().unwrap();
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => {
                    if position_relative_to_end > 0 {
                        position_relative_to_end -= 1;

                        execute!(stdout, MoveRight(1)).unwrap();
                        stdout.flush().unwrap();
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    if buff.len() == 0 || position_relative_to_end == buff.len() {
                        continue;
                    }

                    if position_relative_to_end == 0 {
                        buff.pop();
                        queue!(stdout, MoveLeft(1)).unwrap();
                        queue!(stdout, Print(" ")).unwrap();
                        queue!(stdout, MoveLeft(1)).unwrap();
                        stdout.flush().unwrap();
                    } else {
                        let buff_clone = buff.clone();
                        let (left_part, right_part) =
                            buff_clone.split_at(buff.len() - position_relative_to_end);
                        let mut left_part_chars = left_part.chars();

                        if left_part.len() < 2 {
                            buff = right_part.to_string();
                        } else {
                            left_part_chars.next_back();
                            buff = format!("{}{}", left_part_chars.as_str(), right_part);
                        }

                        queue!(stdout, MoveLeft(1)).unwrap();
                        queue!(stdout, Print(right_part)).unwrap();
                        queue!(stdout, Print(" ")).unwrap();
                        queue!(stdout, MoveLeft(right_part.len() as u16 + 1)).unwrap();

                        stdout.flush().unwrap();
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Delete,
                    ..
                }) => {
                    if buff.len() == 0 || position_relative_to_end == 0 {
                        continue;
                    }

                    if position_relative_to_end == buff.len() {
                        let mut chars = buff.chars();
                        chars.next();
                        buff = chars.as_str().to_string();
                        queue!(stdout, Print(buff.clone())).unwrap();
                        queue!(stdout, Print(" ")).unwrap();
                        queue!(stdout, MoveLeft(buff.len() as u16 + 1)).unwrap();
                        stdout.flush().unwrap();
                    } else {
                        let buff_clone = buff.clone();
                        let (left_part, mut right_part) =
                            buff_clone.split_at(buff.len() - position_relative_to_end);
                        let mut right_part_chars = right_part.chars();

                        if right_part.len() < 2 {
                            buff = left_part.to_string();

                            right_part = "";
                        } else {
                            right_part_chars.next();
                            right_part = right_part_chars.as_str();
                            buff = format!("{}{}", left_part, right_part_chars.as_str());
                        }

                        queue!(stdout, Print(right_part)).unwrap();
                        queue!(stdout, Print(" ")).unwrap();
                        queue!(stdout, MoveLeft(right_part.len() as u16 + 1)).unwrap();

                        stdout.flush().unwrap();
                    }

                    position_relative_to_end -= 1;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(char),
                    ..
                }) => {
                    if position_relative_to_end == 0 {
                        buff.push(char);
                        queue!(stdout, Print(char)).unwrap();
                    } else {
                        // add char to buff at length - relativeToEndPosition
                        let mut new_buff = String::new();
                        let mut i = 0;
                        for c in buff.chars() {
                            if i == buff.len() - position_relative_to_end {
                                new_buff.push(char);
                            }
                            new_buff.push(c);
                            i += 1;
                        }

                        buff = new_buff.clone();

                        // split buff at length - relativeToEndPosition
                        let (_, right_part) =
                            new_buff.split_at(buff.len() - position_relative_to_end - 1);
                        queue!(stdout, Print(right_part)).unwrap();

                        // move cursor back to where it was
                        for _ in 1..right_part.len() {
                            execute!(stdout, MoveLeft(1)).unwrap();
                        }
                    }

                    stdout.flush().unwrap();
                }
                _ => {
                    continue;
                }
            }
        };
    }
}
