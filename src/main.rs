pub mod app; 
pub mod schedule;
pub mod prompts;

fn main() {
    let mut app_data = app::startup();
    
    loop {
        if app::run(&mut app_data) == false {
            println!("Are you sure you want to exit the app? (input y to confirm)");
            
            if let Some(true) = app::console::yes_or_no() {
                println!("Thanks for using automato-p!");
                break;
            }
        }
    }
}