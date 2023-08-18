use crate::{
    app::ScheduleList,
    utils::console, 
    schedule::{Schedule, format, RepeatType, RestType},
};

struct ScheduleCreateResponses {
    name: String,
    work_duration: String,
    rest_duration: String,
    repeat_type: (String, String),
    rest_type: (String, String, String),
}

impl ScheduleCreateResponses {
    const REPEAT_INFINITE: String = String::from("Infinite");
    const REPEAT_FINITE: String = String::from("Finite");

    const LONG_REST: String = String::from("Long Rest");
    const STANDARD_REST: String = String::from("No Long Rest");

    fn new() -> ScheduleCreateResponses {
        ScheduleCreateResponses { 
            name: String::new(),
            work_duration: String::new(),
            rest_duration: String::new(),
            repeat_type: (String::new(), String::new()),
            rest_type: (String::new(), String::new(), String::new()),
        }
    }
}

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

fn prompt(schedule_list: &mut ScheduleList) -> Option<ScheduleCreateResponses> {
    let mut responses = ScheduleCreateResponses::new();

    let mut i = 0;
    while i < SCHEDULE_QUESTIONS.len() {
        console::clear();

        println!("{} (type 'BACK' at any point to go to previous question/menu)", SCHEDULE_QUESTIONS[i]);
        let response = console::get_input_trimmed();

        if response.eq_ignore_ascii_case("back") {
            if i == 0 {
                return None;
            } else {
                i -= 1;
                continue;
            }
        }

        match i {
            0 => responses.name = response,
            1 => responses.work_duration = response,
            2 => responses.rest_duration = response,
            3 => {
                if response.eq("1") {
                    responses.repeat_type.0 = ScheduleCreateResponses::REPEAT_FINITE.clone();
                } else if response.eq("2") {
                    responses.repeat_type.0 = ScheduleCreateResponses::REPEAT_INFINITE.clone();
                    i += 1;
                } else {
                    panic!("Invalid response: must be 1 or 2");
                }
            }
            4 => responses.repeat_type.1 = response,
            5 => {
                if response.eq_ignore_ascii_case("y") {
                    responses.rest_type.0 = ScheduleCreateResponses::LONG_REST;
                } else if response.eq_ignore_ascii_case("n") {
                    responses.repeat_type.0 = ScheduleCreateResponses::STANDARD_REST;
                    i += 2;
                } else {
                    panic!("Invalid response: y or n");
                }
            }
            6 => responses.rest_type.1 = response,
            7 => responses.rest_type.2 = response,
            _ => panic!("How did we get here?"),
        }

        i += 1;
    }

    Some(responses)
}

//all ScheduleCreateResponses should be able to be converted into a Schedule
fn convert_to_schedule(responses: &ScheduleCreateResponses) -> Schedule {
    Schedule {
        name: responses.name,
        work_duration: format::hhmmss_to_dur(&responses.work_duration),
        rest_duration: format::hhmmss_to_dur(&responses.rest_duration),
        repeat_type: match &responses.repeat_type.0 {
            &ScheduleCreateResponses::REPEAT_FINITE => RepeatType::Finite(responses.repeat_type.1.parse().expect("repeat_type.1 should be parseable to u32")),
            &ScheduleCreateResponses::REPEAT_INFINITE => RepeatType::Infinite,
            other => panic!("Could not convert '{other}' to Schedule repeat type"),
        },
        rest_type: match &responses.rest_type.0 {
            &ScheduleCreateResponses::LONG_REST => RestType::LongRest {
                blocks_per_long_rest: responses.rest_type.1.parse().unwrap(),
                long_rest_duration: format::hhmmss_to_dur(&responses.rest_type.2),
            },
            &ScheduleCreateResponses::STANDARD_REST => RestType::Standard,
            other => panic!("Could not convert '{other}' to Schedule rest type"),
        }
    }
}

pub fn start(schedule_list: &mut ScheduleList) -> Option<Schedule> {
    match prompt(schedule_list) {
        Some(responses) => Some(schedule_list.push(convert_to_schedule(&responses))),
        None => None,
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::slice::Iter;

    #[test]
    fn schedules_should_save_to_file() {
        let list = ScheduleList::new();

        let responses = ScheduleCreateResponses {
            name: String::from("Test"),
            work_duration: String::from("60:0"), 
            rest_duration: String::from("0:1"),
            repeat_type: (String::from(ScheduleCreateResponses::REPEAT_INFINITE), String::new()), 
            rest_type: (ScheduleCreateResponses::LONG_REST, String::from("8"), String::from("2:0:0")),
        };

        
    }
}