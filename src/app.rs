use std::{slice::{Iter, IterMut}, io::{BufWriter, self, Write}};
use crate::{save_load, schedule::{self, Schedule}, utils::console};

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
    pub fn get(&self, index: usize) -> &Schedule {
        self.0.get(index).unwrap()
    }

    fn get_mut(&mut self, index: usize) -> &mut Schedule {
        self.0.get_mut(index).unwrap()
    }

    pub fn start_schedule(&self, index: usize) {
        self.0.get(index).unwrap().start();
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

    pub fn iter_mut(&mut self) -> IterMut<'_, Schedule> {
        self.0.iter_mut()
    }

    pub fn display_list(&self) {
        //overly complicated because i can
        let mut writer = BufWriter::new(io::stdout());

        for (i, schedule) in self.iter().enumerate() {
            write!(writer, "{i}: {}", schedule.get_details()).unwrap();
        }

        writer.flush().unwrap();
    }
}