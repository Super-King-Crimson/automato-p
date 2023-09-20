use std::path::Path;

use crate::app::{console, AppData, BACK_CHARACTERS, EXPECT_VERIFIED};

const APP_PROMPTS: [&str; 1] = [
    "Change the app's audio"
];

fn prompt_change_audio() -> Option<Option<String>> {
    let response;

    loop {
        let r = console::get_input_trimmed_exclude(&[&["NONE"], &BACK_CHARACTERS[..]].concat(), false);

        match r {
            Ok(res) => {
                response = res;
                break;
            }
            Err(i) => {
                if i == 0 {
                    return Some(None);
                } else {
                    return None;
                }
            }
        }
    }

    let path = Path::new(&response);

    if path.exists() {
        Some(Some(response))
    } else {
        None
    }
}

fn prompt(app_data: &mut AppData) {
    loop {
        console::clear();
        println!("What would you like to configure about automato-p?");
        for (i, prompt) in APP_PROMPTS.iter().enumerate() {
            println!("{i}: {prompt}");
        }

        loop {
            let response = console::get_input_trimmed_exclude(&BACK_CHARACTERS, false);
    
            if response.is_err() {
                return;
            }
    
            let response = response.expect(EXPECT_VERIFIED);
    
            match response.as_ref() {
                "0" => {
                    println!("Please type the global path to the sound you want to play when an alarm ends.");
                    println!("If you no longer want to play a sound when an alarm ends, type the word NONE.");

                    loop {
                        match prompt_change_audio() {
                            Some(Some(path)) => {
                                app_data.set_sound_path(Some(path));
                                println!("Successfully changed path. The sound at that path will be played whenever an alarm ends.");
                            }
                            Some(None) => {
                                app_data.set_sound_path(None);
                                println!("Succesfully changed path. A sound will no longer play when an alarm ends.")
                            },
                            None => {
                                println!("Path not found, please try again");
                                continue;
                            }
                        }

                        break;
                    }
                }
                _ => {
                    println!("'{response}' is not a valid response.");
                }
            }
            
            println!("Would you like to continue changing automato-p's settings? (y/n)");
                    
            if let Some(true) = console::yes_or_no() {
                continue;
            }

            break;
        }
    }
}

pub fn start(app_data: &mut AppData) {
    prompt(app_data);
}