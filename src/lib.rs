pub mod schedule;

use std::io::{self, BufRead, Write, stdout};
use serde::{Deserialize, Serialize};
use serde_json;

pub fn clear_console() {
    print!("{esc}c", esc = 27 as char);
    stdout().flush().unwrap();
}   

pub fn startup() {
    todo!("FIGURE OUT HOW TO SERDE STUFF INTO A FILE");
}

pub fn run() {
    clear_console();

    println!("Welcome to your automatic pomodoro timer!");

    println!("What would you like to do?");
    
    println!("0: Start a schedule");
    println!("1: Create a new schedule");
    println!("2: Modify a pre-existing schedule");
    println!("3: Configure app defaults");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    clear_console();

    match input.trim().parse() {
        Ok(0_u8) => todo!(),
        Ok(1) => {
            schedule::create_new_schedule();
        },
        Ok(2) => todo!(),
        _ => panic!("invalid"),
    }
}