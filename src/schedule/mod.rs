pub mod format;

use crate::app::console;
use std::{time::Duration, thread, fmt::Display};
use serde::{Serialize, Deserialize}; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestType {
    LongRest {
        blocks_per_long_rest: u32,
        long_rest_duration: Duration,
    },
    Standard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatType {
    Infinite,
    Finite(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub name: String,
    pub work_duration: Duration,
    pub rest_duration: Duration,
    
    pub repeat_type: RepeatType,
    pub rest_type: RestType,
}

const QUARTER_SECOND: Duration = Duration::from_millis(250);
const CONGRATS_TIME: Duration = Duration::from_millis(5000);
impl Schedule {
    pub fn pomodoro() -> Schedule {
        Schedule { 
            name: String::from("Pomodoro"), 
            work_duration: Duration::from_secs(60 * 25), 
            rest_duration: Duration::from_secs(60 * 5), 
            repeat_type: RepeatType::Finite(8), 
            rest_type: RestType::LongRest { blocks_per_long_rest: 4, long_rest_duration: Duration::from_secs(60 * 30) }
        }
    }

    pub fn start(&self, alarm_path: Option<&str>) {
        let mut dur = self.work_duration;
        let mut working = true;
        let mut block_count = 1;

        console::clear();
        console::hide_cursor();
        
        println!("Working block 1");

        loop {
            console::move_cursor_to(0, 1);
            print!("{}", format::dur_to_hhmmss(dur));
            console::flush();

            thread::sleep(QUARTER_SECOND);
            
            match dur.checked_sub(2 * QUARTER_SECOND) {
                Some(new_dur) => dur = new_dur + QUARTER_SECOND,
                None => {
                    working = !working;

                    if let Some(path) = alarm_path {
                        let result = console::play_sound(path);
                        thread::sleep(Duration::from_secs(1));

                        match result {
                            Ok(mut proc) => {
                                if let Ok(Some(status)) = proc.try_wait() {
                                    if !status.success() {
                                        eprintln!("Sound failed to play: \
                                        check to make sure your sound path is correct");
                                        thread::sleep(Duration::from_secs(2)); 
                                    }
                                } else {
                                    panic!("Error attempting to... wait? Don't ask me, I'm puzzled");
                                }
                            }
                            Err(_) => {
                                eprintln!("Sound failed to play: \
                                check to make sure mpg123 is installed");
                                thread::sleep(Duration::from_secs(2));
                            }
                        }
                    }

                    console::clear();

                    if working {
                        println!("Working block {}", block_count);
                        dur = self.work_duration;
                    } else {
                        
                        block_count += 1;

                        if let RepeatType::Finite(repeats) = self.repeat_type {
                            if block_count > repeats {
                                println!("Congratulations, you've completed your schedule! 🎉🎉🎉");
                                thread::sleep(CONGRATS_TIME);
                                break;
                            }
                        }

                        if let RestType::LongRest { blocks_per_long_rest, long_rest_duration } = self.rest_type {
                            if block_count % blocks_per_long_rest == 1 && block_count != 1 {
                                println!("Congratulations on completing {} blocks! Here's a deserved long break:",
                                    if block_count == blocks_per_long_rest { blocks_per_long_rest.to_string() } 
                                    else { format!("another {blocks_per_long_rest}") }
                                );
                                
                                console::hide_cursor();
                                dur = long_rest_duration;
                                continue;
                            }
                        }
                        
                        dur = self.rest_duration;

                        println!("Rest block {}", block_count - 1);
                    }
                }
            }
        }
    }

    fn get_total_duration(&self) -> Option<Duration> {
        match self.repeat_type {
            RepeatType::Finite(blocks) => {
                let total_work = self.work_duration * blocks;
                let mut total_rest = self.rest_duration * (blocks - 1);

                if let RestType::LongRest { blocks_per_long_rest, long_rest_duration } = self.rest_type {
                    let long_rests = (blocks - 1) / blocks_per_long_rest;

                    total_rest += long_rest_duration * long_rests;
                    total_rest -= self.rest_duration * long_rests;
                }

                Some(total_work + total_rest)
            }
            RepeatType::Infinite => None,
        }
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{name}: {work_dur} work, {rest_dur} rest{rest_type_details}{repeat_type_details}",
        
            name = self.name,
            work_dur = format::dur_to_xhxmxs(self.work_duration),
            rest_dur = format::dur_to_xhxmxs(self.rest_duration),
            rest_type_details = match self.rest_type {
                RestType::LongRest { blocks_per_long_rest, long_rest_duration } => {
                    format!(", {} long rest after {} blocks", format::dur_to_xhxmxs(long_rest_duration), blocks_per_long_rest)
                }
                _ => String::new(),
            },
            repeat_type_details = match self.repeat_type {
                RepeatType::Finite(blocks) => format!(", {} blocks long ({})", blocks, format::dur_to_xhxmxs(self.get_total_duration().unwrap())),
                _ => String::new(),
            }
        )
    }
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    fn dur_close_enough(dur1: Duration, dur2: Duration, threshold_ms: u128) -> bool {
        let ms1 = dur1.as_millis();
        let ms2 = dur2.as_millis();

        ms2.abs_diff(ms1) < threshold_ms
    }

    fn pomodoro() -> Schedule {
        Schedule { 
            name: String::from("Pomodoro"), 
            work_duration: Duration::from_secs(25*60), 
            rest_duration: Duration::from_secs(5*60), 
            rest_type: RestType::LongRest { blocks_per_long_rest: 4, long_rest_duration: Duration::from_secs(30*60) }, 
            repeat_type: RepeatType::Finite(8),
        }
    }

    fn test() -> Schedule {
        Schedule { 
            name: String::from("test"), 
            work_duration: Duration::from_secs(1), 
            rest_duration: Duration::from_secs(1), 
            repeat_type: RepeatType::Infinite,
            rest_type: RestType::Standard,
        }
    }
    
    fn test_bounded() -> Schedule {
        let mut test = test();
        test.repeat_type = RepeatType::Finite(2);

        test
    }

    #[test]
    fn schedule_should_last_close_to_its_duration() {
        let schedule = test_bounded();
        let schedule_duration = schedule.get_total_duration().unwrap() + CONGRATS_TIME;

        //I aint tryna wait that long
        assert!(schedule_duration.as_millis() < 10000, "actual duration was {}", schedule_duration.as_millis());

        let before = SystemTime::now();
        
        thread::spawn(move || {
            schedule.start(None);
        }).join().unwrap();

        let passed_time = {
            let now = SystemTime::now();
            now.duration_since(before).unwrap()
        };

        assert!(dur_close_enough(
            passed_time,
            schedule_duration,
            1000
        ), "passed_time: {passed_time:?}, duration: {schedule_duration:?}");
    }
}