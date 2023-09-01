pub mod app; 
pub mod schedule;
pub mod prompts;

use app::save_load::SCHEDULE_PATH;

fn main() {
    let mut schedules = app::startup(SCHEDULE_PATH.to_string());

    loop {
        if let false = app::run(&mut schedules) {
            break;
        }
    }
}