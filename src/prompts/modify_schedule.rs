use crate::{app::{console, AppData}, schedule::{Schedule, RepeatType::*, RestType::{*, self}, format::try_hhmmss_to_dur}};

const CHANGE_OPTIONS: [&str; 6] = [
    "Name",
    "Work Duration",
    "Rest Duration",
    "Repeat Type",
    "Rest Type",
    "Delete this schedule",
];

fn prompt_create_long_rest() -> Option<RestType> {
    let blocks_per_long_rest;
    let long_rest_duration;

    loop {
        println!("How many blocks until your long rest?");
        let response = console::get_input_trimmed();
    
        if response.eq_ignore_ascii_case("BACK") {
            return None
        }
        
        let num_blocks = response.parse::<u32>();
        
        if let Err(_) = num_blocks {
            println!("{response} is not a number, try again.");
        } else {
            blocks_per_long_rest = num_blocks.unwrap();
            break;
        }
    }

    loop {
        println!("And how long should your long rest be? (HH:MM:SS)");
    
        let response = console::get_input_trimmed();
        if response.eq_ignore_ascii_case("BACK") {
            return None
        }

        let dur = try_hhmmss_to_dur(&response);
        
        if let None = dur {
            println!("That isn't a valid duration, try again.")
        } else {
            long_rest_duration = dur.unwrap();
            break;
        }
    }

    Some(LongRest { blocks_per_long_rest, long_rest_duration })
}

fn change_schedule(schedule: &Schedule, option_index: usize) -> Option<Option<Schedule>> {
    let mut new_schedule = schedule.clone();

    if option_index == 0 {
        println!("What would you like to change the name to?");

        let response = console::get_input_trimmed();
        println!("Successfully changed name from {} to {}.", schedule.name, &response);
        new_schedule.name = response;
    } else if option_index == 1 {
        println!("What would you like to change the work duration to?");
        
        let response = console::get_input_trimmed();

        if let Some(dur) = try_hhmmss_to_dur(&response) {
            println!("Successfully changed work duration.");
            new_schedule.work_duration = dur;
        }
    } else if option_index == 2 {
        println!("What would you like to change the rest duration to?");
        let response = console::get_input_trimmed();

        if let Some(dur) = try_hhmmss_to_dur(&response) {
            println!("Successfully changed rest duration.");
            new_schedule.work_duration = dur;
        }
    } else if option_index == 3 {
        println!("Would you like to change your repeat type? (current repeat type: {}) (y/n)", if let Infinite = schedule.repeat_type {"Infinite"} else {"Finite"}); 
        let response = console::get_input_trimmed();

        match response.to_ascii_lowercase().as_ref() {
            "y" => {
                match new_schedule.repeat_type {
                    Infinite => {
                        println!("How many work rest blocks should your schedule repeat before it ends on its own?");
                        
                        loop {
                            let response = console::get_input_trimmed();

                            if response.eq_ignore_ascii_case("BACK") {
                                return None
                            } 
                            
                            let num_blocks = response.parse::<u32>();

                            if let Err(_) = num_blocks  {
                                println!("{response} is not a number, try again.")
                            } else {
                                println!("Successfully changed repeat type.");
                                new_schedule.repeat_type = Finite(num_blocks.unwrap());
                                break;
                            }
                        }
                    }
                    Finite(_) => {
                        println!("Successfully changed repeat type.");
                        new_schedule.repeat_type = Infinite;
                    }
                }
            }
            "n" => {
                match new_schedule.repeat_type {
                    Infinite => println!("This schedule cannot have anything changed about it's repeat type, as it is infinite."),
                    Finite(a) => {
                        println!("Currently your schedule ends on its own after {a} work/rest cycles, what would you like to change it to?");

                        loop {
                            let response = console::get_input_trimmed();

                            if response.eq_ignore_ascii_case("BACK") {
                                return None;
                            }
    
                            let cycles = response.parse::<u32>();
    
                            if let Err(_) = cycles {
                                println!("{response} is not a valid number, please retry");    
                            } else {
                                println!("Successfully changed the max blocks.");
                                new_schedule.repeat_type = Finite(cycles.unwrap());
                                break;
                            }
                        }
                    }
                }
            }
            _ => {
                println!("Invalid response.");
                return None;
            }
        }
    } else if option_index == 4 {
        match new_schedule.rest_type {
            Standard => {
                println!("Would you like to change your schedule to have a long rest every couple of blocks? (y/n)");

                if let Some(yes) = console::yes_or_no() {
                    if yes {
                        let long_rest = prompt_create_long_rest()?;

                        new_schedule.rest_type = long_rest;
                        println!("Successfully changed rest type.");
                    }
                } else {
                    println!("Invalid response.");
                    return None
                }
            }
            LongRest {..} => {
                println!("What would you like to change about your rest type?");
                println!("0: Change to different rest type");
                println!("1: Change rest type details");

                let response = console::get_input_trimmed();

                match response.as_ref() {
                    "0" => {
                        println!("Would you like to change your rest type to Standard, with no long rests?");
                        if let Some(yes) = console::yes_or_no() {
                            if yes {
                                new_schedule.rest_type = Standard;

                                println!("Successfully changed rest type to standard");
                            } else {
                                println!("Okay. Returning to main menu, as there are no other rest types.");
                            }
                        } else {
                            println!("Invalid response");
                            return None;
                        }
                        
                    },
                    "1" => {
                        println!("a");
                    },
                    _ => return None,
                }
            }
        }
    } else if option_index == 5 {
        loop {
            println!("Are you sure you want to delete {}? (type yes to confirm)", schedule.name);

            if let Some(true) = console::yes_or_no() {
                return Some(None);
            }
        }
    } else {
        panic!("Expected option index to be less than the length of change options");
    }

    Some(Some(new_schedule))
}

fn prompt(app_data: &mut AppData) {
    println!("Which schedule would you like to modify?");

    app_data.display_schedule_list();
    let response = console::get_input_trimmed();

    if response.eq_ignore_ascii_case("back") {
        return;
    }

    let schedule_index = match response.parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            console::clear();
            println!("'{response}' is not a valid schedule. Please try again.");
            prompt(app_data);
            return
        },
    };

    if schedule_index >= app_data.num_schedules() {
        console::clear();
        println!("'{response}' is not a valid schedule. Please try again.");
        prompt(app_data);
        return
    }

    let schedule = app_data.get_schedule(schedule_index);
    
    println!("What would you like to change about {}?", schedule.name);
    for (i, opt) in CHANGE_OPTIONS.iter().enumerate() {
        println!("{i}: {opt}");
    }

    let response = console::get_input_trimmed();

    if response.eq_ignore_ascii_case("back") {
        return;
    } else {
        match response.parse::<usize>() {
            Ok(option_index) if option_index < CHANGE_OPTIONS.len() => {
                match change_schedule(schedule, option_index) {
                    Some(Some(replacement)) => {
                        app_data.replace_schedule(schedule_index, replacement);
                        println!("Successfully updated schedule.");
                    }
                    Some(None) => {
                        app_data.remove_schedule(schedule_index);
                        println!("Successfully removed schedule.");
                    }
                    _ => return,
                }
            },
            _ => {
                println!("'{response}' is not a valid response. Please try again.");
                prompt(app_data);
                return;
            }
        }
    }

    println!("Would you like to continue changing your schedule? (y/n)");
    match console::yes_or_no() {
        Some(true) => {
            prompt(app_data);
            return;
        }
        Some(false) => println!("Returning to main menu..."),
        None => println!("Invalid response, assumed 'no'"),
    }
}

pub fn start(app_data: &mut AppData) {
    println!("Type BACK at any point to go back to the main menu");
    prompt(app_data);
}