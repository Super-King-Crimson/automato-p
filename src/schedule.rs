use std::{time::Duration, thread, io::{Write, stdout, BufRead}, error::Error, ops::Mul};
use crossterm::{cursor, terminal, ExecutableCommand};
use serde::{Serialize, Deserialize};
use crate::clear_console;

fn format_dur_hms(dur: Duration) -> String {
    let mut secs = dur.as_secs();

    let mut mins = secs / 60;
    secs %= 60;

    let hours = mins / 60;
    mins %= 60;

    let secs_str = if secs == 0 {String::new()} else {format!("{secs}s")};
    let mins_str = if mins == 0 {String::new()} else {format!("{mins}m")};
    let hours_str = if hours == 0 {String::new()} else {format!("{hours}h")};

    format!("{hours_str}{mins_str}{secs_str}")
}

fn format_dur(dur: Duration) -> String {
    let mut secs = dur.as_secs();

    let mut mins = secs / 60;
    secs %= 60;

    let hours = mins / 60;
    mins %= 60;

    let secs_str = if secs < 10 {format!("0{secs}")} else {secs.to_string()};
    let mut mins_str = mins.to_string();

    let mut hours_str = String::new();
    if hours != 0 {
        hours_str = format!("{hours}:");
        mins_str = if mins < 10 {format!("0{mins_str}")} else {format!("{mins_str}")};
    }

    format!("{}{}:{}", hours_str, mins_str, secs_str)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RepeatType {
    LongRest {
        blocks_per_long_rest: u32,
        long_rest_duration: Duration,
    },
    Standard,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaxBlocks {
    Infinite,
    Finite(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub name: String,
    pub work_duration: Duration,
    pub rest_duration: Duration,
    
    pub repeat_type: RepeatType,

    pub max_blocks: MaxBlocks,

    pub block_count: u32,
    pub working: bool,
}

impl Schedule {
    pub fn default() -> Schedule {
        Schedule {
            name: String::from("Pomodoro"),
            
            work_duration: Duration::from_secs(2),
            rest_duration: Duration::from_secs(1),

            repeat_type: RepeatType::LongRest { blocks_per_long_rest: 4, long_rest_duration: Duration::from_secs(2) },
            max_blocks: MaxBlocks::Finite(8),

            block_count: 1,
            working: true,
        }
    }

    pub fn start(mut self) {
        let mut dur = self.work_duration;
        let quarter_second = Duration::from_millis(250);

        //Clear the console, hide the cursor
        clear_console();
        stdout().execute(cursor::Hide).unwrap();
        stdout().flush().unwrap();
        
        println!("Working block 1");

        loop {
            stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
            stdout().execute(cursor::MoveTo(0, 1)).unwrap();
            print!("{}", format_dur(dur));
            stdout().flush().unwrap();

            thread::sleep(quarter_second);
            
            //TODO: try to fix this so it displays the initial time for 0.75 secs, and 0:00 for 0.25 secs
            match dur.checked_sub(2 * quarter_second) {
                Some(new_dur) => dur = new_dur + quarter_second,
                None => {
                    self.working = !self.working;
                    clear_console();

                    if self.working {
                        println!("Working block {}", self.block_count);
                        dur = self.work_duration;
                    } else {
                        self.block_count += 1;

                        if let MaxBlocks::Finite(repeats) = self.max_blocks {
                            if self.block_count > repeats {
                                println!("Congratulations, you've completed your schedule! 🎉🎉🎉");
                                thread::sleep(Duration::from_secs(5));
                                break;
                            }
                        }

                        if let RepeatType::LongRest { blocks_per_long_rest, long_rest_duration } = self.repeat_type {
                            if self.block_count % blocks_per_long_rest == 1 && self.block_count != 1 {
                                println!("Congratulations on completing {} blocks! Here's a deserved long break:",
                                    if self.block_count == blocks_per_long_rest { blocks_per_long_rest.to_string() } 
                                    else { format!("another {blocks_per_long_rest}") }
                                );
                                
                                stdout().execute(cursor::Hide).unwrap();
                                dur = long_rest_duration;
                                continue;
                            }
                        }
                        
                        dur = self.rest_duration;

                        println!("Rest block {}", self.block_count - 1);
                    }

                    stdout().execute(cursor::Hide).unwrap();
                }
            }
        }
    }

    fn get_total_duration(&self) -> Option<Duration> {
        match self.max_blocks {
            MaxBlocks::Finite(blocks) => {
                let total_work = self.work_duration * blocks;
                let mut total_rest = self.rest_duration * (blocks - 1);

                if let RepeatType::LongRest { blocks_per_long_rest, long_rest_duration } = self.repeat_type {
                    let long_rests = (blocks - 1) / blocks_per_long_rest;

                    total_rest += long_rest_duration * long_rests;
                    total_rest -= self.rest_duration * long_rests;
                }

                Some(total_work + total_rest)
            }
            MaxBlocks::Infinite => None,
        }
    }

    pub fn display_stats(&self) {
        println!("{name}: {work_dur} work, {rest_dur} rest{repeat_type_details}{max_blocks_details}", 
            name = self.name,
            work_dur = format_dur_hms(self.work_duration),
            rest_dur = format_dur_hms(self.rest_duration),
            repeat_type_details = match self.repeat_type {
                RepeatType::LongRest { blocks_per_long_rest, long_rest_duration } => {
                    format!(", {} long rest after {} blocks", format_dur_hms(long_rest_duration), blocks_per_long_rest)
                }
                _ => String::new(),
            },
            max_blocks_details = match self.max_blocks {
                MaxBlocks::Finite(blocks) => format!(", {} blocks long ({})", blocks, format_dur_hms(self.get_total_duration().unwrap())),
                _ => String::new(),
            }
        );
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

fn hhmmss_to_duration(str: &str) -> Duration {
    let mut secs = 0u64;

    for (s, i) in str.split(':').rev().zip(0u32..) {
        secs += s.parse::<u64>().unwrap() * 60u64.checked_pow(i).unwrap();
    }

    Duration::from_secs(secs)
}

pub fn create_new_schedule() -> Schedule {
    let mut input = std::io::stdin().lock();
    let mut output = stdout().lock();

    let mut name = String::new();
    let mut work_duration = Duration::ZERO;
    let mut rest_duration = Duration::ZERO;

    let mut max_blocks = MaxBlocks::Infinite;

    let mut blocks_per_long_rest = 0u32;
    let mut long_rest_duration = Duration::ZERO;
    let mut repeat_type = RepeatType::Standard;

    for mut i in 0..SCHEDULE_QUESTIONS.len() {
        clear_console();

        println!("{}", SCHEDULE_QUESTIONS[i]);
        let mut response = String::new();
        input.read_line(&mut response);
        response = response.trim().to_string();

        match i { 
            0 => name.push_str(response.trim()),
            1 => work_duration = hhmmss_to_duration(&response),
            2 => rest_duration = hhmmss_to_duration(&response),
            3 => {
                if response.eq("1") {
                    ()
                } else if response.eq("2") {
                    i += 1;
                } else {
                    panic!("brother you have entered an invalid character");
                }
            }
            4 => max_blocks = MaxBlocks::Finite(response.parse().unwrap()),
            5 => if response.eq_ignore_ascii_case("y") {
                ()
            } else if response.eq_ignore_ascii_case("n") {
                i += 2;
            } else {
                panic!("brother you have entered an invalid character");
            }
            6 => blocks_per_long_rest = response.parse().unwrap(),
            7 => {
                long_rest_duration = hhmmss_to_duration(&response);
                repeat_type = RepeatType::LongRest { blocks_per_long_rest, long_rest_duration };
            }
            _ => panic!("How did we get here?"),
        }
    }

    Schedule { 
        name, 
        work_duration,
        rest_duration,
        repeat_type, 
        max_blocks, 
        block_count: 1, 
        working: true 
    }
}
