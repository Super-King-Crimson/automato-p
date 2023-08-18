use crate::{
    app::{ScheduleList, Schedules},
    utils::{console, wait}, 
    schedule::Schedule,
};

struct ScheduleCreateResponses {
    name: String,
    work_duration: String,
    rest_duration: String,
    repeat_type: (String, String),
    long_rest: (String, String, String),
}

impl ScheduleCreateResponses {
    fn new() -> ScheduleCreateResponses {
        ScheduleCreateResponses { 
            name: String::new(),
            work_duration: String::new(),
            rest_duration: String::new(),
            repeat_type: (String::new(), String::new()),
            long_rest: (String::new(), String::new(), String::new()),
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

pub fn prompt(schedule_list: &mut ScheduleList) -> Option<ScheduleCreateResponses> {
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
                    responses.repeat_type.0 = String::from("finite");
                } else if response.eq("2") {
                    responses.repeat_type.0 = String::from("infinite");
                    i += 1;
                } else {
                    panic!("Invalid response: must be 1 or 2");
                }
            }
            4 => responses.repeat_type.1 = response,
            5 => {
                if response.eq_ignore_ascii_case("y") {
                    responses.long_rest.0 = String::from("yes");
                } else if response.eq_ignore_ascii_case("n") {
                    responses.long_rest.0 = String::from("no");
                    i += 2;
                } else {
                    panic!("Invalid response: y or n");
                }
            }
            6 => responses.long_rest.1 = response,
            7 => responses.long_rest.2 = response,
            _ => panic!("How did we get here?"),
        }

        i += 1;
    }

    Some(responses)
}

pub fn convert(responses: ScheduleCreateResponses) -> Schedule {
    let name = responses.name;
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

pub fn modify<L: Schedules>(schedule_list: &mut L) {
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
    use std::slice::Iter;
    
    struct MockList(Vec<Schedule>);

    impl MockList {
        fn new() -> MockList {
            MockList(Vec::new())
        }
    }

    impl Schedules for MockList {
        fn get(&self, index: usize) -> &Schedule {
            &self.0[index]
        }

        fn push(&mut self, schedule: Schedule) {
            self.0.push(schedule);
        }

        fn insert(&mut self, index: usize, schedule: Schedule) {
            self.0.insert(index, schedule);
        }

        fn remove(&mut self, index: usize) {
            self.0.remove(index);
        }

        fn replace(&mut self, index: usize, replacement: Schedule) {
            self.0[index] = replacement;
        }

        fn iter(&self) -> Iter<'_, Schedule> {
            self.0.iter()
        }
    }

    #[test]
    #[should_panic]
    fn schedules_should_be_prompt_created() {
        let list = MockList::new();

        create(&mut)
    }
}