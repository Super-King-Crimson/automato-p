pub mod app; 
pub mod save_load;
pub mod utils;
pub mod schedule;
pub mod prompts;

fn main() {
    let mut schedules = app::startup(save_load::SCHEDULE_PATH).unwrap();

    loop {
        app::run(&mut schedules);
    }
}