use crate::app::{console, AppData};

pub fn start(app_data: &mut AppData) {
    println!("Which schedule would you like to start?");
    app_data.display_schedule_list();
    println!("B: Back");

    let response = {
        let response = console::get_input_trimmed();

        if response.eq_ignore_ascii_case("B") {
            return;
        }

        response.parse().unwrap()
    };

    app_data.start_schedule(response);
}