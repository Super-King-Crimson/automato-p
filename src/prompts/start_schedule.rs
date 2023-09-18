use crate::app::{console, AppData, EXPECT_VERIFIED};

pub fn start(app_data: &mut AppData) {
    println!("Which schedule would you like to start?");
    app_data.display_schedule_list();

    loop {
        let response = console::get_input_trimmed_exclude(&["B"], false);

        if response.is_err() {
            return;
        }

        let response = response.expect(EXPECT_VERIFIED);

        match response.parse::<usize>() {
            Ok(index) if index < app_data.num_schedules() => app_data.start_schedule(index),
            _ => println!("{response} is an invalid response, try again"),
        }
    }
}