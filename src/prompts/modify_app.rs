const APP_PROMPTS: [&str; 1] = [
    "Change the app's audio"
];

fn prompt() {
    println!("What would you like to configure about automato-p?");
    for (i, prompt) in APP_PROMPTS.iter().enumerate() {
        println!("{i}: {prompt}");
    }

        // Command::new("mpg123")
        // .arg("user/aura.mp3")
        // .output().unwrap();

    todo!("Parse match also use this code to play sound with mpg123");
}

pub fn start() {

}