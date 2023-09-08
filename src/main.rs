pub mod app; 
pub mod schedule;
pub mod prompts;

fn main() {
    let mut app_data = app::startup();
    
    loop {
        if app::run(&mut app_data) == false {
            break;
        }
    }
}