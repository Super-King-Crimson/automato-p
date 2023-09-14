use std::path::Path;

use crate::app::{console, AppData};

const APP_PROMPTS: [&str; 1] = [
    "Change the app's audio"
];

fn prompt_change_audio() -> Option<Option<String>> {
    println!("Please type the global path to the sound you want to play when an alarm ends.");
    println!("If you no longer want to play a sound when an alarm ends, type the word NONE.");
    let response = console::get_input_trimmed();

    if response.eq("NONE") {
        return Some(None);
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
        println!("What would you like to configure about automato-p? (Type BACK to go back to the previous men8)");
        for (i, prompt) in APP_PROMPTS.iter().enumerate() {
            println!("{i}: {prompt}");
        }

        let response = console::get_input_trimmed();

        if response.eq_ignore_ascii_case("BACK") {
            break;
        }

        match response.as_ref() {
            "0" => {
                match prompt_change_audio() {
                    Some(Some(path)) => {
                        app_data.set_sound_path(Some(path));
                        println!("Successfully changed path. The sound at that path will be played whenever an alarm ends.");
                    }
                    Some(None) => {
                        app_data.set_sound_path(None);
                        println!("Succesfully changed path. A sound will no longer play when an alarm ends.")
                    },
                    None => println!("Path not found, please try again"),
                }

                println!("Would you like to continue changing automato-p's settings? (y/n)");
                if let None | Some(false) = console::yes_or_no() {
                    break;
                }
            }
            _ => {
                console::clear();
                println!("Invalid response.");
                break;
            }
        }
    }
}

pub fn start(app_data: &mut AppData) {
    prompt(app_data);
}