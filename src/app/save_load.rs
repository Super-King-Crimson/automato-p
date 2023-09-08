use std::{fs::{OpenOptions, File}, io::{self, BufRead, BufWriter, BufReader, Lines, Write}, path::{PathBuf, Path}};
use crate::schedule::Schedule;

use super::app_settings::AppSettings;

const SCHEDULE_PATH: &str = "./user/schedules.txt";
const SETTINGS_PATH: &str = "./user/settings.json";

const EXPECT_VALID_UTF8: &str = "Line should contain valid UTF-8";
const EXPECT_FILE: &str = "File should exist, as it is created at the beginning of the program's start";
const EXPECT_WRITE: &str = "File should be able to be written to";
const EXPECT_VALID_JSON: &str = "Schedule file should contain valid JSON";
const EXPECT_VALID_TO_JSON: &str = "Value should be convertible to JSON";

fn read_lines_from_file<P: AsRef<Path>>(path: P) -> Result<Lines<BufReader<File>>, io::Error> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

fn read_from_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut str = String::new();

    for line in read_lines_from_file(path)? {
        str.push_str(&line?);
        str.push('\n');
    }

    Ok(str)
}

fn append_to_file<P: AsRef<Path>>(path: P, contents: &str) -> Result<(), io::Error> {
    let file = OpenOptions::new().append(true).open(path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(contents.as_bytes())?;
    Ok(())
}

fn write_to_file<P: AsRef<Path>>(path: P, contents: &str) -> Result<(), io::Error> {
    let file = OpenOptions::new().write(true).truncate(true).open(path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(contents.as_bytes())?;
    Ok(())
}

fn is_file_empty<P: AsRef<Path>>(path: P) -> Result<bool, io::Error> {
    let lines = read_lines_from_file(path);

    if lines?.next().is_none() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub struct SaveLoad {
    schedule_path: PathBuf,
    settings_path: PathBuf,
}

impl SaveLoad {
    pub fn new() -> SaveLoad {
        if let Ok(true) | Err(_) = is_file_empty(&SCHEDULE_PATH) {
            SaveLoad::init_schedule_file(&SCHEDULE_PATH);
        }

        if let Ok(true) | Err(_) = is_file_empty(&SETTINGS_PATH) {
            SaveLoad::init_settings_file(&SETTINGS_PATH);
        }

        SaveLoad {
            schedule_path: SCHEDULE_PATH.into(),
            settings_path: SETTINGS_PATH.into(),
        } 
    }

    pub fn read_schedules(&self) -> Vec<Schedule> {
        read_lines_from_file(&self.schedule_path).expect(EXPECT_FILE)
            .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
            .collect()
    }

    pub fn append_schedule(&self, schedule: &Schedule) {
        let mut json = serde_json::to_string(schedule).unwrap();
        json.push('\n');

        append_to_file(&self.schedule_path, &json).expect(EXPECT_VALID_UTF8);
    }

    pub fn insert_schedule(&self, index: usize, schedule: &Schedule) {
        let lines = read_lines_from_file(&self.schedule_path).expect(EXPECT_FILE);
        let mut buf = String::new();

        for (i, line) in lines.enumerate() {
            let line = line.expect(EXPECT_VALID_UTF8);

            if i == index {
                buf.push_str(&serde_json::to_string(schedule).expect(EXPECT_VALID_JSON));
            } 
            
            buf.push_str(&line);
        }

        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.schedule_path).unwrap();
        file.write_all(buf.as_bytes()).expect(EXPECT_VALID_UTF8);
    }

    pub fn remove_schedule(&self, index: usize) {
        let lines = read_lines_from_file(&self.schedule_path).expect(EXPECT_FILE);
        let mut buf = String::new();

        for (i, line) in lines.enumerate() {
            let line = line.expect(EXPECT_VALID_UTF8);

            if i == index {
                continue;
            } else {
                buf.push_str(&line);
            }
        }

        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.schedule_path).expect(EXPECT_FILE);
        file.write_all(buf.as_bytes()).expect(EXPECT_WRITE);
    }

    pub fn replace_schedule(&self, index: usize, replacement: &Schedule) {
        let lines = read_lines_from_file(&self.schedule_path).expect(EXPECT_FILE);
        let mut buf = String::new();

        for (i, line) in lines.enumerate() {
            let line = line.expect(EXPECT_VALID_UTF8);

            if i == index {
                buf.push_str(&serde_json::to_string(replacement).expect(EXPECT_VALID_JSON));
            } else {
                buf.push_str(&line);
            }
        }

        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.schedule_path).expect(EXPECT_FILE);
        file.write_all(buf.as_bytes()).expect(EXPECT_VALID_UTF8);
    }

    pub fn read_settings(&self) -> AppSettings {
        let contents = read_from_file(&self.settings_path).expect(EXPECT_FILE);
        serde_json::from_str(&contents).expect(EXPECT_VALID_JSON)
    }

    pub fn change_settings(&self, new_settings: AppSettings) {
        let json = serde_json::to_string(&new_settings).expect(EXPECT_VALID_TO_JSON);
        write_to_file(&self.settings_path, &json).expect(EXPECT_FILE);
    }

    fn init_settings_file(path: &str) {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(path).expect(EXPECT_FILE);
        let mut writer = BufWriter::new(file);

        let json = serde_json::to_string_pretty(&AppSettings::default()).expect(EXPECT_VALID_TO_JSON);
            
        writer.write_all(json.as_bytes()).expect(EXPECT_VALID_UTF8);
        writer.write(b"\n").expect(EXPECT_WRITE);
    }

    fn init_schedule_file(path: &str) {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(path).expect(EXPECT_FILE);
        let mut writer = BufWriter::new(file);

        let json = serde_json::to_string(&Schedule::pomodoro()).expect(EXPECT_VALID_TO_JSON);

        writer.write_all(json.as_bytes()).expect(EXPECT_VALID_UTF8);
        writer.write(b"\n").expect(EXPECT_WRITE);
    }
}
