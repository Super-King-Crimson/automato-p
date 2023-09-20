use std::time::Duration;

use crate::{
    app::{console, AppData, BACK_CHARACTERS, EXPECT_VERIFIED},
    schedule::{format, RepeatType, RestType, Schedule},
};

enum RepeatTypeResponse {
    Infinite,
    Finite { blocks: String },
}
use RepeatTypeResponse as RpTR;

enum RestTypeResponse {
    Standard,
    LongRest {
        blocks_per_long_rest: String,
        long_rest_duration: String,
    },
}
use RestTypeResponse as RsTR;

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

        let q = SCHEDULE_QUESTIONS[question_index];

        println!("{q}");
        let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false);

        if response.is_err() {
            match previous_questions.pop() {
                Some(prev) => {
                    question_index = prev;
                    continue;
                }
                None => return None,
            }
        }

        let response = response.expect(EXPECT_VERIFIED);

        if Some(&question_index) != previous_questions.last() {
            previous_questions.push(question_index);
        }


        match question_index {
            0 => responses.name = response,
            1 => responses.work_duration = response,
            2 => responses.rest_duration = response,
            3 => {
                if response.eq("1") {
                    responses.repeat_type = RpTR::Finite {
                        blocks: String::new(),
                    }
                } else if response.eq("2") {
                    responses.repeat_type = RpTR::Infinite;
                    question_index += 1;
                } else {
                    continue; //restarts question ask, incrementer is at the end
                }
            }
            4 => {
                if let RpTR::Finite { blocks } = &mut responses.repeat_type {
                    *blocks = response;
                }
            }
            5 => {
                if response.eq_ignore_ascii_case("y") {
                    responses.rest_type = RsTR::LongRest {
                        blocks_per_long_rest: String::new(),
                        long_rest_duration: String::new(),
                    }
                } else if response.eq_ignore_ascii_case("n") {
                    responses.rest_type = RsTR::Standard;
                    question_index += 2;
                } else {
                    continue;
                }
            }
            6 => {
                if let RsTR::LongRest {
                    blocks_per_long_rest,
                    ..
                } = &mut responses.rest_type
                {
                    *blocks_per_long_rest = response;
                }
            }
            7 => {
                if let RsTR::LongRest {
                    long_rest_duration, ..
                } = &mut responses.rest_type
                {
                    *long_rest_duration = response;
                }
            }
            _ => unreachable!(),
        }

        question_index += 1;
    }

    Some(responses)
}

fn try_convert_to_schedule(responses: ScheduleCreateResponses) -> Result<Schedule, String> {
    let mut issues = String::new();

    let work_duration = format::try_hhmmss_to_dur(&responses.work_duration);
    if work_duration.is_none() {
        issues += &format!(
            "'{}' could not be converted into an HH:MM:SS duration - duration of work block\n",
            &responses.work_duration
        );
    }

    let rest_duration = format::try_hhmmss_to_dur(&responses.rest_duration);
    if work_duration.is_none() {
        issues += &format!(
            "'{}' could not be converted into an HH:MM:SS duration - duration of rest block\n",
            &responses.rest_duration
        );
    }

    let mut finite_blocks: Option<Option<u32>> = None;
    
    if let RpTR::Finite { blocks } = responses.repeat_type { 
        finite_blocks = { 
            match blocks.parse() { 
                Ok(num) => Some(Some(num)), 
                Err(_) => { 
                    issues += &format!("'{blocks}' must be a positive integer - number of blocks before schedule stops\n" ); 

                    Some(None)
                }
            } 
        }
    }

    let (mut long_rest_blocks, mut long_rest_dur): (Option<Option<u32>>, Option<Option<Duration>>) = (None, None);

    if let RsTR::LongRest { blocks_per_long_rest, long_rest_duration } = responses.rest_type {
        long_rest_blocks = if let Ok(b) = blocks_per_long_rest.parse() {
            Some(Some(b))
        } else {
            issues += &format!("'{blocks_per_long_rest}' must be a positive integer - number of blocks per long rest\n");

            Some(None)
        };

        long_rest_dur = if let Some(d) = format::try_hhmmss_to_dur(&long_rest_duration) {
            Some(Some(d))
        } else {
            issues += &format!("'{long_rest_duration}' could not be converted into an HH:MM:SS duration - long rest duration\n");

            Some(None)
        }
    }

    if issues.len() == 0 {
        Ok(
            Schedule {
                name: responses.name,
                work_duration: work_duration.expect(EXPECT_VERIFIED),
                rest_duration: rest_duration.expect(EXPECT_VERIFIED),
                repeat_type: if finite_blocks.is_none() {
                    RepeatType::Infinite
                } else {
                    RepeatType::Finite(finite_blocks.expect(EXPECT_VERIFIED).expect(EXPECT_VERIFIED))
                },
                rest_type: if long_rest_blocks.is_none() {
                    RestType::Standard
                } else {
                    RestType::LongRest {
                        blocks_per_long_rest: long_rest_blocks.expect(EXPECT_VERIFIED).expect(EXPECT_VERIFIED), 
                        long_rest_duration: long_rest_dur.expect(EXPECT_VERIFIED).expect(EXPECT_VERIFIED)
                    }
                },
            }
        )
    } else {
        Err(issues)
    }
}

pub fn start(app_data: &mut AppData) {
    loop {
        if let Some(responses) = prompt() {
            let result = try_convert_to_schedule(responses);
    
            match result {
                Ok(schedule) => app_data.push_schedule(schedule),
                Err(issues) => {
                    println!("Failed to convert responses to a schedule, as the following issues were present:");
                    for (line, i) in issues.lines().zip(1u8..) {
                        println!("Issue {i}: {line}");
                    }
    
                    println!("Would you like to try to make a schedule again? (Input 'yes' to confirm)");
    
                    if let Some(true) = console::yes_or_no() {
                        continue;
                    }
                }
            }
        }

        break;
    }
}
