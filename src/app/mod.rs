pub mod save_load;
pub mod console;
pub mod app_settings;
pub mod error;
pub mod schedule_list;

use crate::{prompts, schedule::Schedule};
use save_load::SaveLoad;

use schedule_list::ScheduleList; 
use app_settings::AppSettings;

pub struct AppData {
    pub app_settings: AppSettings,
    schedule_list: ScheduleList,
    save_load: SaveLoad,
}

impl AppData {
    pub fn push_schedule(&mut self, schedule: Schedule) {
        self.save_load.append_schedule(&schedule);
        self.schedule_list.push(schedule);
    }

    pub fn insert_schedule(&mut self, index: usize, schedule: Schedule) {
        self.save_load.insert_schedule(index, &schedule);
        self.schedule_list.insert(index, schedule);
    }

    pub fn remove_schedule(&mut self, index: usize) {
        self.save_load.remove_schedule(index);
        self.schedule_list.remove(index);
    }

    pub fn replace_schedule(&mut self, index: usize, replacement: Schedule) {
        self.save_load.replace_schedule(index, &replacement);
        self.schedule_list.replace(index, replacement)
    }

    pub fn display_schedule_list(&self) {
        self.schedule_list.display_list();
    }

    pub fn start_schedule(&self, index: usize) {
        self.schedule_list.start_schedule(index, self.app_settings.get_sound_path());
    }

    pub fn num_schedules(&self) -> usize {
        self.schedule_list.len()
    }

    pub fn get_schedule(&self, index: usize) -> &Schedule {
        self.schedule_list.get(index)
    }
}

pub fn startup() -> AppData {
    let save_load = SaveLoad::new();

    let schedule_list = ScheduleList::from(save_load.read_schedules());
    let app_settings = save_load.read_settings();

    AppData {
        save_load,
        schedule_list,
        app_settings,
    }
}

pub fn run(mut app_data: &mut AppData) -> bool {
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
        Ok(0_u8) => prompts::start_schedule::start(&mut app_data),
        Ok(1) => prompts::create_schedule::start(&mut app_data),
        Ok(2) => prompts::modify_schedule::start(&mut app_data),
        Ok(3) => prompts::modify_app::start(&mut app_data),
        Ok(4) => {
            return false;
        }
        _ => panic!("invalid"),
    };

    true
}