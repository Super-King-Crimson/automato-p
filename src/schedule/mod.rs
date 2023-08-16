pub mod prompt;
mod format;

use crate::utils::console;
use std::{time::Duration, thread};
use serde::{Serialize, Deserialize}; 

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

        console::clear();
        console::hide_cursor();
        
        println!("Working block 1");

        loop {
            console::move_cursor_to(0, 0);
            print!("{}", format::dur_to_xhxmxs(dur));
            console::flush();

            thread::sleep(quarter_second);
            
            //TODO: try to fix this so it displays the initial time for 0.75 secs, and 0:00 for 0.25 secs
            match dur.checked_sub(2 * quarter_second) {
                Some(new_dur) => dur = new_dur + quarter_second,
                None => {
                    self.working = !self.working;
                    console::clear();

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
                                
                                console::hide_cursor();
                                dur = long_rest_duration;
                                continue;
                            }
                        }
                        
                        dur = self.rest_duration;

                        println!("Rest block {}", self.block_count - 1);
                    }

                    
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

    pub fn prompt_print(&self) -> String {
        format!("{name}: {work_dur} work, {rest_dur} rest{repeat_type_details}{max_blocks_details}", 
            name = self.name,
            work_dur = format::dur_to_xhxmxs(self.work_duration),
            rest_dur = format::dur_to_xhxmxs(self.rest_duration),
            repeat_type_details = match self.repeat_type {
                RepeatType::LongRest { blocks_per_long_rest, long_rest_duration } => {
                    format!(", {} long rest after {} blocks", format::dur_to_xhxmxs(long_rest_duration), blocks_per_long_rest)
                }
                _ => String::new(),
            },
            max_blocks_details = match self.max_blocks {
                MaxBlocks::Finite(blocks) => format!(", {} blocks long ({})", blocks, format::dur_to_xhxmxs(self.get_total_duration().unwrap())),
                _ => String::new(),
            }
        )
    }
}
