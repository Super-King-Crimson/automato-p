use crate::{app::{console, AppData, EXPECT_VERIFIED, BACK_CHARACTERS, B_FOR_BACK}, schedule::{Schedule, RepeatType::*, RestType::{*, self}, format::try_hhmmss_to_dur}};

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
        println!("How many blocks should you have to work through to get your long rest?");
        let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS,false).ok()?;
        
        if let Ok(blocks) = response.parse() {
            blocks_per_long_rest = blocks;
            break; 
        } else {
            println!("{response} is not a number, try again.");
        }
    }

    loop {
        println!("And how long should your long rest be? (HH:MM:SS)");
    
        let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false).ok()?;

        let dur = try_hhmmss_to_dur(&response);
        
        if let Some(d) = dur {
            long_rest_duration = d;
            break;
        } else {
            println!("That isn't a valid duration, try again.")
        }
    }

    Some(LongRest { blocks_per_long_rest, long_rest_duration })
}

fn change_schedule(schedule: &Schedule, option_index: usize) -> Option<Option<Schedule>> {
    let mut new_schedule = schedule.clone();

    if option_index == 0 {
        println!("What would you like to change the name to?");

        let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false).ok()?;

        println!("Successfully changed name from {} to {}.", schedule.name, &response);
        
        new_schedule.name = response;
    } else if option_index == 1 {
        println!("What would you like to change the work duration to?");

        loop {
            let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false).ok()?;

            if let Some(dur) = try_hhmmss_to_dur(&response) {
                new_schedule.work_duration = dur;
                println!("Successfully changed work duration.");
                break;
            } else {
                println!("Could not convert '{response}' to a valid duration, please enter a valid HH:MM:SS duration below");
            }
        }
    } else if option_index == 2 {
        println!("What would you like to change the rest duration to?");

        loop {
            let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false).ok()?;

            if let Some(dur) = try_hhmmss_to_dur(&response) {
                new_schedule.work_duration = dur;
                println!("Successfully changed rest duration.");
                break;
            } else {
                println!("Could not convert '{response}' to a valid duration, please enter a valid HH:MM:SS duration below");
            }
        }
    } else if option_index == 3 {
        println!("Would you like to change your repeat type? (current repeat type: {}) (yes to confirm)", if let Infinite = schedule.repeat_type {"Infinite"} else {"Finite"}); 
        
        loop {
            match console::yes_or_no() {
                Some(true) => {
                    match new_schedule.repeat_type {
                        Infinite => {
                            println!("How many work rest blocks should your schedule repeat before it ends on its own?");
    
                            loop {
                                let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false).ok()?;
    
                                let num_blocks = response.parse::<u32>();
    
                                if let Ok(blocks) = num_blocks  {
                                    println!("Successfully changed repeat type.");
                                    new_schedule.repeat_type = Finite(blocks);
                                } else { 
                                    println!("{response} is not a number, try again.");
                                    continue;
                                }
    
                                break;
                            }
                        }
                        Finite(_) => {
                            println!("Successfully changed repeat type.");
                            new_schedule.repeat_type = Infinite;
                        }
                    }
                }
                Some(false) => {
                    match new_schedule.repeat_type {
                        Infinite => println!("This schedule cannot have anything changed about it's repeat type, as it is infinite."),
                        Finite(a) => {
                            println!("Currently your schedule ends on its own after {a} work/rest cycles, what would you like to change it to?");
    
                            loop {
                                let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false).ok()?;
                                
                                let cycles = response.parse::<u32>();
                                
                                if let Ok(c) = cycles {
                                    println!("Successfully changed the max blocks.");
                                    new_schedule.repeat_type = Finite(c);
                                } else {
                                    println!("{response} is not a valid number, please retry");
                                    continue;
                                }
                                    
                                break;
                            }
                        }
                    }
                }
                _ => {
                    println!("Invalid response, please enter 'y' or 'n' to answer the question");
                    continue;
                }
            }

            break;
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

                loop {
                    let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false).ok()?;
    
                    match response.as_ref() {
                        "0" => {
                            println!("Would you like to change your rest type to Standard, with no long rests?");
                            loop {
                                if let Some(yes) = console::yes_or_no() {
                                    if yes {
                                        new_schedule.rest_type = Standard;
        
                                        println!("Successfully changed rest type to standard.");
                                    } else {
                                        println!("Returning to main menu, as there are no other rest types.");
                                        return None;
                                    }
                                } else {
                                    println!("Invalid input, please input 'y' or 'n'");
                                    continue;
                                }
                                
                                break;
                            }    
                        }, 
                        "1" => if let Some(new_long_rest) = prompt_create_long_rest() {
                            new_schedule.rest_type = new_long_rest;
                        } else {
                            return None;
                        },
                        _ => {
                            println!("Invalid response, please input '1' or '2'");
                            continue;
                        }
                    }

                    break; 
                }
            }
        }
    } else if option_index == 5 {
        loop {
            println!("Are you sure you want to delete {}? (input yes to confirm)", schedule.name);

            if let Some(true) = console::yes_or_no() {
                return Some(None);
            }
        }
    } else {
        unreachable!();
    }

    Some(Some(new_schedule))
}

fn prompt(app_data: &mut AppData) {
    'main: loop {
        println!("Which schedule would you like to modify?");   

        app_data.display_schedule_list();
        let schedule_index;
        loop {
            let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false); 

            if response.is_err() {
                return;
            }   

            let response = response.expect(EXPECT_VERIFIED);    

            schedule_index = match response.parse::<usize>() {
                Ok(num) if num < app_data.num_schedules() => num,
                _ => {
                    println!("'{response}' is not a valid schedule. Please choose a different schedule.");
                    continue;
                },
            };  

            break;
        }   

        let schedule = app_data.get_schedule(schedule_index);

        'question: loop {
            println!("What would you like to change about {}?", schedule.name);
            for (i, opt) in CHANGE_OPTIONS.iter().enumerate() {
                println!("{i}: {opt}");
            }   
    
            loop { 
                let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false); 
    
                if response.is_err() {
                    continue 'main;
                } 
    
                let response = response.expect(EXPECT_VERIFIED);
    
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
                            None => {
                                continue 'question;
                            },
                        }
                    },
                    _ => {
                        println!("'{response}' is not a valid response. Please choose a different option below:");
                        continue;
                    }
                }
    
                break;
            }
            
            break;
        }

        println!("Would you like to continue changing your schedules? (yes to confirm)");
        if let Some(true) = console::yes_or_no() {
            console::clear();
            continue;
        }
        
        break;
    }
}

pub fn start(app_data: &mut AppData) {
    println!("{B_FOR_BACK}");
    prompt(app_data);
}