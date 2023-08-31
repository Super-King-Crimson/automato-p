use crate::app::{console, ScheduleList};

pub fn start(schedule_list: &mut ScheduleList) {
    println!("Which schedule would you like to start?");
    schedule_list.display_list();
    println!("B: Back");

    let response = {
        let response = console::get_input_trimmed();

        if response.eq_ignore_ascii_case("b") {
            return;
        }

        response.parse().unwrap()
    };

    schedule_list.get(response).start();
}

const MODIFY_OPTIONS: [&str; 5] = [
    "Change name",
    "Change work/rest/long rest settings",
    "Add or remove schedule duration",
    "Delete schedule",
    "Return to menu",
];

pub fn modify(schedule_list: &mut ScheduleList) {
    println!("Which schedule would you like to modify?");
    schedule_list.display_list();

    let response = console::get_input_trimmed().parse().unwrap();

    let schedule = schedule_list.get(response);

    loop {
        console::clear();
        println!("What would you like to change about {}?", schedule.name);

        for (i, opt) in MODIFY_OPTIONS.iter().enumerate() {
            println!("{i}: {opt}");
        }

        match console::get_input_trimmed().as_str() {
            "0" => {
                println!("What would you like to change it to?");
                let (old_name, new_name) = (&schedule.name, console::get_input_trimmed()); 

                println!("Successfully changed schedule name from {old_name} to {}. Press any key to continue.", &new_name);

                let mut new_schedule = schedule.clone();
                new_schedule.name = new_name;
                
                schedule_list.replace(response, new_schedule);

                console::wait_for_key_press();
                break;
            }
            "1" => (),
            "2" => (),
            "3" => {
                console::clear();
                println!("Are you sure you want to delete schedule {}? (y/n)", schedule.name);

                if &console::get_input_trimmed() == "y" {
                    println!("Deleted {}", schedule.name);
                    schedule_list.remove(response);
                }

                break;
            }
            "4" => break,
            _ => {
                println!("Invalid response, press any key to retry");
                console::wait_for_key_press();
            }
        }
    }
}
