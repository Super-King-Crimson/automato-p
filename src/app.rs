use crate::{utils::console, schedule::{self, Schedule}, save_load};

static mut SCHEDULES: Vec<Schedule> = vec![];

fn display_schedules() {
    unsafe {
        for (i, schedule) in SCHEDULES.iter().enumerate() {
            println!("{i}: {}", schedule.prompt_print());
        }
    }
}

pub fn startup() {
    unsafe { 
        SCHEDULES = save_load::load_schedules();
    }
}

pub fn remove_schedule(index: usize) {
    unsafe {
        SCHEDULES.remove(index);
    }
}

pub fn add_schedule(schedule: Schedule) {
    unsafe {
        SCHEDULES.push(schedule)
    }
}

pub fn run() {
    console::clear();

    println!("Welcome to your automatic pomodoro timer!");

    println!("What would you like to do?");
    
    println!("0: Start a schedule");
    println!("1: Create a new schedule");
    println!("2: Modify a pre-existing schedule");
    println!("3: Configure app defaults");

    let input = console::get_input_trimmed();

    console::clear();

    match input.parse() {
        Ok(0_u8) => {
            println!("Which schedule would you like to start?");
            display_schedules();
            let response = console::get_input_trimmed();
            let parsed: usize = response.parse().unwrap();
            loop {
                println!("You chose response {parsed}.");
            }
        },
        Ok(1) => schedule::prompt::create(),
        Ok(2) => todo!(),
        _ => panic!("invalid"),
    };
}