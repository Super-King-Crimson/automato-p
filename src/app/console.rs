use std::{io::{self, Write}, process::{Command, Stdio, Child}};
use crossterm::{event::{self, Event, KeyEvent, KeyEventKind}, cursor, terminal, ExecutableCommand};

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
    io::stdout().flush().unwrap();
}

pub fn clear_line() {
    io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
}

pub fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf
}

pub fn get_input_trimmed() -> String {
    get_input().trim().to_string()
}

pub fn move_cursor_to(x: u16, y: u16) {
    io::stdout().execute(cursor::MoveTo(x, y)).unwrap();
}

pub fn wait_for_key_press() -> KeyEvent {
    terminal::enable_raw_mode().unwrap();

    loop {
        let event = event::read().unwrap();

        if let Event::Key(event) = event {
            if event.kind == KeyEventKind::Press {
                terminal::disable_raw_mode().unwrap();
                return event;
            }
        }
    }
}

pub fn yes_or_no() -> Option<bool> {
    let response = get_input_trimmed();

    match response.as_ref() {
        "y" | "yes" => Some(true),
        "n" | "no" => Some(false),
        _ => None
    }
}

pub fn flush() {
    io::stdout().flush().unwrap();
}

pub fn play_sound(path: &str) -> Result<Child, io::Error> {
    Command::new("mpg123")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

pub fn get_input_trimmed_exclude(excludes: &[&str], case_sensitive: bool) -> Result<String, usize> {
    let input = get_input_trimmed();

    for (i, excl) in excludes.iter().enumerate() {
        if case_sensitive {
            if input.eq(excl) {
                return Err(i);
            }
        } else {
            if input.eq_ignore_ascii_case(excl) {
                return Err(i);
            }
        }
    }

    return Ok(input);
}