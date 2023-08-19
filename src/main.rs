pub mod app; 
pub mod schedule;
pub mod prompts;

use app::save_load::SCHEDULE_PATH;

fn main() {
    let mut schedules = app::startup(SCHEDULE_PATH.to_string()).unwrap();

    loop {
        app::run(&mut schedules);
    }
}