pub mod app; 
pub mod save_load;
pub mod utils;
pub mod schedule;

fn main() {
    let mut schedules = app::startup();
    
    loop {
        app::run(&mut schedules);
    }
}