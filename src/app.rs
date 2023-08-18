use std::slice::Iter;
use crate::{
    save_load, 
    schedule::Schedule, 
    utils::console,
    prompts,
};

pub struct ScheduleList(Vec<Schedule>);

impl ScheduleList {
    fn get_mut(&mut self, index: usize) -> &mut Schedule {
        self.0.get_mut(index).unwrap()
    }

    pub fn new() -> ScheduleList {
        ScheduleList(Vec::new())
    }

    pub fn start_schedule(&self, index: usize) {
        self.get(index).start();
    }

    pub fn display_list(&self) {
        self.iter().enumerate().for_each(|(i, sch)| println!("{i}: {sch}"));
    }
    
    pub fn get(&self, index: usize) -> &Schedule {
        self.0.get(index).unwrap()
    }

    pub fn push(&mut self, schedule: Schedule) {
        self.0.push(schedule);
    }

    pub fn insert(&mut self, index: usize, schedule: Schedule) {
        self.0.insert(index, schedule);
    }

    pub fn remove(&mut self, index: usize) {
        save_load::delete_schedule(index);
        self.0.remove(index);
    }

    pub fn replace(&mut self, index: usize, replacement: Schedule) {
        save_load::replace_schedule(index, &replacement);
        *self.get_mut(index) = replacement;
    }

    pub fn iter(&self) -> Iter<'_, Schedule> {
        self.0.iter()
    }
}

pub fn startup() -> ScheduleList {
    ScheduleList(save_load::load_schedules())
}

pub fn run(schedule_list: &mut ScheduleList) {
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
        Ok(0_u8) => prompts::start_schedule::start(schedule_list),
        Ok(1) => prompts::create_schedule::start(schedule_list),
        Ok(2) => todo!(),
        _ => panic!("invalid"),
    };
}