use std::{fs, io::{Write, BufWriter, BufRead, BufReader}};
use super::schedule::Schedule;
use crate::app;

const PATH_TO_FILE: &str = "./schedules.txt";

fn read_schedules_from_file() -> Vec<Schedule> {
    let file = fs::File::open(PATH_TO_FILE).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| serde_json::from_str(&line.unwrap()).unwrap()).collect()
}

fn write_schedule_to_file(data: &Schedule) {
    let mut json = serde_json::to_string(data).unwrap();
    json.push('\n');
    let mut file = BufWriter::new(
        fs::OpenOptions::new().write(true).append(true).open(PATH_TO_FILE).unwrap());
    file.write(json.as_bytes()).unwrap();

    file.flush().unwrap();
}

pub fn save_schedule(schedule: Schedule) {
    write_schedule_to_file(&schedule);
    app::add_schedule(schedule);
}

pub fn load_schedules() -> Vec<Schedule> {
    read_schedules_from_file()
}
