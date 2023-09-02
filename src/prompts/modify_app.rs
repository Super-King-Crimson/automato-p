use crate::app::console;

const APP_PROMPTS: [&str; 1] = [
    "Change the app's audio"
];

fn prompt_change_audio() {
    println!("Please type the path to the sound you want to play when an alarm ends.");
    let path = console::get_input_trimmed();
    todo!("Do something to get this into the hands of schedule");
}

fn prompt() {
    println!("What would you like to configure about automato-p?");
    for (i, prompt) in APP_PROMPTS.iter().enumerate() {
        println!("{i}: {prompt}");
    }

    let response = console::get_input_trimmed();
    match response.as_ref() {
        "0" => prompt_change_audio(),
        _ => {
            println!("Invalid response.");
            return;
        }
    }
}

pub fn start() {
    
}