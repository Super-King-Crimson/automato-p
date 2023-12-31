pub mod app_settings;
pub mod console;
pub mod error;
pub mod save_load;
pub mod schedule_list;

pub const B_FOR_BACK: &str = "Type BACK at any point to return to the previous menu.";
pub const BACK_CHARACTERS: [&str; 1] = ["BACK"];

pub const EXPECT_VERIFIED: &str = "Value has already been verified to exist";


use crate::{prompts, schedule::Schedule};
use save_load::SaveLoad;

use app_settings::AppSettings;
use schedule_list::ScheduleList;

pub struct AppData {
    app_settings: AppSettings,
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
        self.schedule_list
            .start_schedule(index, self.get_sound_path());
    }

    pub fn num_schedules(&self) -> usize {
        self.schedule_list.len()
    }

    pub fn get_schedule(&self, index: usize) -> &Schedule {
        self.schedule_list.get(index)
    }

    pub fn get_sound_path(&self) -> Option<&str> {
        self.app_settings.sound_path.as_deref()
    }

    pub fn set_sound_path(&mut self, new_path: Option<String>) {
        self.app_settings.sound_path = new_path;
        self.save_load.save_settings(&self.app_settings);
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

pub fn run(app_data: &mut AppData) -> bool {
    console::clear();

    loop {
        println!("Welcome to your automatic pomodoro timer, automato-p!");
        println!("{B_FOR_BACK}");

        println!("What would you like to do?");
        println!("0: Start a schedule");
        println!("1: Create a new schedule");
        println!("2: Modify a pre-existing schedule");
        println!("3: Change app settings");
        println!("4: Exit app");

        let input = {
            let res = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false);

            match res {
                Ok(r) => r,
                Err(_) => "4".to_string(), //If they type 'B' at the main menu, treat it as exiting the app
            }
        };

        console::clear();

        match input.parse::<u8>() {
            Ok(i @ 0..=4) => {
                match i {
                    0 => prompts::start_schedule::start(app_data),
                    1 => prompts::create_schedule::start(app_data),
                    2 => prompts::modify_schedule::start(app_data),
                    3 => prompts::modify_app::start(app_data),
                    4 => return false,
                    _ => unreachable!()
                }

                break;
            }
            _ => println!("{input} is not a valid response, try again"),
        };
    }

    true
}
