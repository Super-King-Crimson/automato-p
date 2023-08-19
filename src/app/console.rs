use std::{io::{self, Write}, time::{Duration, SystemTime}};
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
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}

pub fn hide_cursor() {
    io::stdout().execute(cursor::Hide).unwrap();
}

pub fn move_cursor_to(x: u16, y: u16) {
    io::stdout().execute(cursor::MoveTo(x, y)).unwrap();
}

pub fn wait_for_key_press() -> Duration {
    let curr_time = SystemTime::now();
    terminal::enable_raw_mode().unwrap();

    loop {
        let event = event::read().unwrap();

        if let Event::Key(KeyEvent {kind: KeyEventKind::Press, ..}) = event {
            break;
        }
    }

    terminal::disable_raw_mode().unwrap();
    SystemTime::now().duration_since(curr_time).unwrap()
}

pub fn flush() {
    io::stdout().flush().unwrap();
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn wait_for_key_press_does_its_thing() {
        wait_for_key_press();
    }
}
