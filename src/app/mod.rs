pub mod save_load;
pub mod console;

use std::{io, slice::Iter, process::Command};
use crate::{
    schedule::Schedule,
    prompts,
};
use save_load::SaveLoad;

pub struct ScheduleList {
    list: Vec<Schedule>, 
    save_load: SaveLoad,
}

impl ScheduleList {
    pub fn from(path_to_file: String) -> ScheduleList {
        let save_load = SaveLoad::create(path_to_file);

        ScheduleList {
            list: save_load.read_schedules(),
            save_load
        }
    }

    pub fn try_from(path_to_file: String) -> Result<ScheduleList, io::Error> {
        let save_load = SaveLoad::try_from(path_to_file)?;

        Ok(ScheduleList {
            list: save_load.read_schedules(),
            save_load
        })
    }

    pub fn start_schedule(&self, index: usize) {
        self.get(index).start();
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn display_list(&self) {
        self.iter().enumerate().for_each(|(i, sch)| println!("{i}: {sch}"));
    }
    
    pub fn get(&self, index: usize) -> &Schedule {
        self.list.get(index).unwrap()
    }

    pub fn push(&mut self, schedule: Schedule) {
        self.save_load.append_schedule(&schedule);
        self.list.push(schedule);
    }

    pub fn insert(&mut self, index: usize, schedule: Schedule) {
        self.save_load.insert_schedule(index, &schedule);
        self.list.insert(index, schedule);
    }

    pub fn remove(&mut self, index: usize) {
        self.save_load.remove_schedule(index);
        self.list.remove(index);
    }

    pub fn replace(&mut self, index: usize, replacement: Schedule) {
        self.save_load.replace_schedule(index, &replacement);
        *self.get_mut(index) = replacement;
    }

    pub fn iter(&self) -> Iter<'_, Schedule> {
        self.list.iter()
    }

    fn get_mut(&mut self, index: usize) -> &mut Schedule {
        self.list.get_mut(index).unwrap()
    }
}

pub fn startup(schedule_file: String) -> ScheduleList {
    ScheduleList::from(schedule_file)
}

pub fn run(schedule_list: &mut ScheduleList) -> bool {
    console::clear();

    println!("Welcome to your automatic pomodoro timer!");

    println!("What would you like to do?");

    println!("0: Start a schedule");
    println!("1: Create a new schedule");
    println!("2: Modify a pre-existing schedule");
    println!("3: Configure app defaults");
    println!("4: Exit app");

    let input = console::get_input_trimmed();

    console::clear();

    match input.parse() {
        Ok(0_u8) => prompts::start_schedule::start(schedule_list),
        Ok(1) => prompts::create_schedule::start(schedule_list),
        Ok(2) => prompts::modify_schedule::start(schedule_list),
        Ok(3) => todo!("FIGURE OUT HOW TO PLAY SOUNDS FROM COMMAND LINE"),
        Ok(4) => {
            return false;
        }
        _ => panic!("invalid"),
    };

    true
}