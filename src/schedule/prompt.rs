use super::*;
use crate::{
    app::ScheduleList,
    utils::{console, wait},
};
use std::time::Duration;

const SCHEDULE_QUESTIONS: [&str; 8] = [
    "What should your new Schedule be named?",
    "How long should your work block be? (HH:MM::SS)",
    "What about your rest block? (HH:MM::SS)",

    "Should your schedule (1)end on its own, or (2)should you have to manually exit? (Answer 1 or 2)",
    "After how many work + rest blocks should your schedule end?",

    "Do you want a long break? (y/n)",
    "After how many work + rest blocks do you want your long break?",
    "How long should your break be? (HH:MM:SS)",
];

pub fn create(schedule_list: &mut ScheduleList) {
    let mut name = String::new();
    let mut work_duration = Duration::ZERO;
    let mut rest_duration = Duration::ZERO;

    let mut max_blocks = MaxBlocks::Infinite;

    let mut blocks_per_long_rest = 0u32;
    let mut long_rest_duration;
    let mut repeat_type = RepeatType::Standard;

    let mut i = 0;
    while i < SCHEDULE_QUESTIONS.len() {
        console::clear();

        println!(
            "{} (type 'BACK' at any point to go to previous question/menu)",
            SCHEDULE_QUESTIONS[i]
        );
        let response = &console::get_input_trimmed();

        if response.eq_ignore_ascii_case("back") {
            if i == 0 {
                return;
            }

            i -= 1;
            continue;
        }

        #[allow(unused)]
        match i {
            0 => name.push_str(response.trim()),
            1 => work_duration = format::hhmmss_to_dur(response),
            2 => rest_duration = format::hhmmss_to_dur(response),
            3 => {
                if response.eq("2") {
                    i += 1;
                } else if !response.eq("1") {
                    panic!("brother you have entered an invalid character");
                }
            }
            4 => max_blocks = MaxBlocks::Finite(response.parse().unwrap()),
            5 => {
                if response.eq_ignore_ascii_case("n") {
                    i += 2;
                } else if !response.eq("y") {
                    panic!("brother you have entered an invalid character");
                }
            }
            6 => blocks_per_long_rest = response.trim().parse().unwrap(),
            7 => {
                long_rest_duration = format::hhmmss_to_dur(response.trim());
                repeat_type = RepeatType::LongRest {
                    blocks_per_long_rest,
                    long_rest_duration,
                };
            }
            _ => panic!("How did we get here?"),
        }

        i += 1;
    }

    let schedule = Schedule {
        name,
        work_duration,
        rest_duration,
        repeat_type,
        max_blocks,
    };

    println!(
        "Schedule {} created! Press any key to continue.",
        schedule.name
    );
    schedule_list.push(schedule);

    thread::sleep(Duration::from_millis(100));
    console::wait_for_key_press();
}

pub fn start(schedule_list: &mut ScheduleList) {
    println!("Which schedule would you like to start?");
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

                println!("Successfully changed schedule name from {old_name} to {}.", &new_name);

                let mut new_schedule = schedule.clone();
                new_schedule.name = new_name;
                
                schedule_list.replace(response, new_schedule);

                wait::for_secs(3);
                break;
            }
            "1" => (),
            "2" => (),
            "3" => {
                console::clear();
                println!(
                    "Are you sure you want to delete schedule {}? (y/n)",
                    schedule.name
                );
                if &console::get_input_trimmed() == "y" {
                    println!("Deleted {}", schedule.name);
                    schedule_list.remove(response);
                }
                //Must break here so we don't try to reaccess a schedule we don't have access to
                break;
            }
            "4" => break,
            _ => {
                println!("Invalid response, press any key to retry");
                wait::for_ms(100);
                console::wait_for_key_press();
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::io::{self, BufRead, BufWriter, Write};
    use std::process::Command;

    #[test]
    #[should_panic]
    fn schedule_should_yeetus_deletus() {
        todo!();
    }
}
