pub mod schedule;

use std::io::{self, BufRead, Write, stdout};

pub fn clear_console() {
    print!("{esc}c", esc = 27 as char);
    stdout().flush().unwrap();
}

pub fn startup() {
    clear_console();
    
    println!("Welcome to your automatic pomodoro timer!");

    println!("What would you like to do?");
    
    println!("0: Start a pomodoro schedule");
    println!("1: Create a new pomodoro schedule");
    println!("2: Modify a pre-existing schedule");

    let mut reader = io::stdin().lock();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    match input.trim().parse() {
        Ok(0_u8) => todo!(),
        Ok(1) => todo!(),
        Ok(2) => {
            clear_console();
            println!("Which one would you like to modify?\n");

            print!("0: ");
            schedule::Schedule::default().display_stats();
            print!("1: ");
            schedule::Schedule::default().display_stats();
            print!("2: ");
            schedule::Schedule::default().display_stats();
            print!("3: ");
            schedule::Schedule::default().display_stats();
            print!("4: ");
            schedule::Schedule::default().display_stats();
        }
        other => panic!("Invalid input {:?}", other),
    }
}