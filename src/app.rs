use std::slice::Iter;
use crate::{
    save_load, 
    schedule::{self, Schedule}, 
    utils::console,
    prompts::{create_schedule, modify_schedule, start_schedule},
};

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
        Ok(0_u8) => schedule::prompt::start(schedule_list),
        Ok(1) => schedule::prompt::create(schedule_list),
        Ok(2) => schedule::prompt::modify(schedule_list),
        _ => panic!("invalid"),
    };
}

pub struct ScheduleList(Vec<Schedule>);

impl ScheduleList {
    fn get_mut(&mut self, index: usize) -> &mut Schedule {
        self.0.get_mut(index).unwrap()
    }
}

impl Schedules for ScheduleList {
    fn get(&self, index: usize) -> &Schedule {
        self.0.get(index).unwrap()
    }

    fn push(&mut self, schedule: Schedule) {
        self.0.push(schedule);
    }

    fn insert(&mut self, index: usize, schedule: Schedule) {
        self.0.insert(index, schedule);
    }

    fn remove(&mut self, index: usize) {
        save_load::delete_schedule(index);
        self.0.remove(index);
    }

    fn replace(&mut self, index: usize, replacement: Schedule) {
        save_load::replace_schedule(index, &replacement);
        *self.get_mut(index) = replacement;
    }

    fn iter(&self) -> Iter<'_, Schedule> {
        self.0.iter()
    }
}

pub trait Schedules {
    fn get(&self, index: usize) -> &Schedule;

    fn push(&mut self, schedule: Schedule);

    fn insert(&mut self, index: usize, schedule: Schedule);

    fn remove(&mut self, index: usize);

    fn replace(&mut self, index: usize, replacement: Schedule);

    fn iter(&self) -> Iter<'_, Schedule>;
        
    fn start_schedule(&self, index: usize) {
        self.get(index).start();
    }

    fn display_list(&self) {
        self.iter().enumerate().for_each(|(i, sch)| println!("{i}: {sch}"));
    }
}