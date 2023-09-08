use crate::{
    app::{console, AppData},
    schedule::{format, RepeatType, RestType, Schedule},
};

enum RepeatTypeResponse {
    Infinite,
    Finite { blocks: String },
}

enum RestTypeResponse {
    Standard,
    LongRest {
        blocks_per_long_rest: String,
        long_rest_duration: String,
    },
}

struct ScheduleCreateResponses {
    name: String,
    work_duration: String,
    rest_duration: String,
    repeat_type: RepeatTypeResponse,
    rest_type: RestTypeResponse,
}

impl ScheduleCreateResponses {
    fn new() -> ScheduleCreateResponses {
        ScheduleCreateResponses {
            name: String::new(),
            work_duration: String::new(),
            rest_duration: String::new(),
            repeat_type: RepeatTypeResponse::Infinite,
            rest_type: RestTypeResponse::Standard,
        }
    }
}

const SCHEDULE_QUESTIONS: [&str; 8] = [
    "What should your new Schedule be named?",
    "How long should your work block be? (HH:MM::SS)",
    "What about your rest block? (HH:MM::SS)",

    "Should your schedule (1) last a fixed amount of time, or (2) continue infinitely until you manually exit? (Answer 1 or 2)",
    "After how many work + rest blocks should your schedule end?",

    "Do you want a long break? (y/n)",
    "After how many work + rest blocks do you want your long break?",
    "How long should your break be? (HH:MM:SS)",
];

fn prompt() -> Option<ScheduleCreateResponses> {
    let mut responses = ScheduleCreateResponses::new();

    let mut question_index = 0;
    let mut previous_questions: Vec<usize> = Vec::new();
    while question_index < SCHEDULE_QUESTIONS.len() {
        console::clear();

        println!(
            "{} (type 'BACK' at any point to go to previous question/menu)",
            SCHEDULE_QUESTIONS[question_index]
        );
        let response = console::get_input_trimmed();

        if response.eq_ignore_ascii_case("back") {
            match previous_questions.pop() {
                Some(prev) => {
                    question_index = prev;
                    continue;
                }
                None => return None,
            }
        }

        previous_questions.push(question_index);

        match question_index {
            0 => responses.name = response,
            1 => responses.work_duration = response,
            2 => responses.rest_duration = response,
            3 => if response.eq("1") {
                responses.repeat_type = RepeatTypeResponse::Finite {
                    blocks: String::new(),
                }
            } else if response.eq("2") {
                responses.repeat_type = RepeatTypeResponse::Infinite;
                question_index += 1;
            } else {
                panic!("Invalid response: must be 1 or 2");
            },
            4 => if let RepeatTypeResponse::Finite { blocks } = &mut responses.repeat_type {
                *blocks = response;
            },
            5 => if response.eq_ignore_ascii_case("y") {
                responses.rest_type = RestTypeResponse::LongRest { blocks_per_long_rest: String::new(), long_rest_duration: String::new() }
            } else if response.eq_ignore_ascii_case("n") {
                responses.rest_type = RestTypeResponse::Standard;
                question_index += 2;
            } else {
                panic!("Invalid response: y or n");
            },
            6 => if let RestTypeResponse::LongRest { blocks_per_long_rest, ..} = &mut responses.rest_type {
                *blocks_per_long_rest = response;
            }
            7 => if let RestTypeResponse::LongRest { long_rest_duration, .. } = &mut responses.rest_type {
                *long_rest_duration = response;
            }
            _ => panic!("How Did You Do This"),
        }

        question_index += 1;
    }

    Some(responses)
}

fn convert_to_schedule(responses: ScheduleCreateResponses) -> Schedule {
    Schedule {
        name: responses.name,
        work_duration: format::hhmmss_to_dur(&responses.work_duration),
        rest_duration: format::hhmmss_to_dur(&responses.rest_duration),
        repeat_type: match responses.repeat_type {
            RepeatTypeResponse::Finite { blocks } => RepeatType::Finite(blocks.parse().unwrap()),
            RepeatTypeResponse::Infinite => RepeatType::Infinite,
        },
        rest_type: match responses.rest_type {
            RestTypeResponse::LongRest {
                blocks_per_long_rest: blocks,
                long_rest_duration: dur,
            } => RestType::LongRest {
                blocks_per_long_rest: blocks.parse().unwrap(),
                long_rest_duration: format::hhmmss_to_dur(&dur),
            },
            RestTypeResponse::Standard => RestType::Standard,
        },
    }
}

pub fn start(app_data: &mut AppData) {
    if let Some(responses) = prompt() {
        let schedule = convert_to_schedule(responses);
        app_data.push_schedule(schedule);
    }
}