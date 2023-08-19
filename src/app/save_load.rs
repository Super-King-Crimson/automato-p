use std::{iter::Iterator, fs::{self, OpenOptions}, io::{Write, BufWriter, BufRead, BufReader}, path::PathBuf};
use crate::schedule::Schedule;

pub const SCHEDULE_PATH: &str = "schedules.txt";

pub struct SaveLoad {
    path: PathBuf
}

impl SaveLoad {
    pub fn try_from(path: String) -> Result<SaveLoad, std::io::Error> {
        match fs::metadata(&path) {
            Ok(_) => Ok(SaveLoad { path: path.into() }),
            Err(e) => Err(e),
        }
    }

    pub fn read_schedules(&self) -> Vec<Schedule> {
        self.iter_lines().map(|line| serde_json::from_str(&line).unwrap()).collect()
    }

    pub fn append_schedule(&self, schedule: &Schedule) {
        let file = OpenOptions::new().write(true).append(true).open(&self.path).unwrap();
        let mut writer = BufWriter::new(file);

        let mut json = serde_json::to_string(schedule).unwrap();
        json.push('\n');

        writer.write_all(json.as_bytes()).unwrap();
        writer.flush().unwrap();
    }

    pub fn insert_schedule(&self, index: usize, schedule: &Schedule) {
        let mut contents = String::new();

        for (i, line) in self.iter_lines().enumerate() {
            if i == index {
                contents.push_str(&serde_json::to_string(schedule).unwrap());
            } 
            
            contents.push_str(&line);
        }

        OpenOptions::new().write(true).open(&self.path).unwrap().write_all(contents.as_bytes()).unwrap();
    }

    pub fn remove_schedule(&self, index: usize) {
        let mut contents = String::new();

        for (i, line) in self.iter_lines().enumerate() {
            if i == index {
                continue;
            } else {
                contents.push_str(&line);
            }
        }

        OpenOptions::new().append(true).open(&self.path).unwrap().write_all(contents.as_bytes()).unwrap();
    }

    pub fn replace_schedule(&self, index: usize, replacement: &Schedule) {
        let mut contents = String::new();

        for (i, line) in self.iter_lines().enumerate() {
            if i == index {
                contents.push_str(&serde_json::to_string(replacement).unwrap());
            } else {
                contents.push_str(&line);
            }
        }

        OpenOptions::new().write(true).open(&self.path).unwrap().write_all(contents.as_bytes()).unwrap();
    }

    fn iter_lines(&self) -> impl Iterator<Item = String> {
        let file = OpenOptions::new().read(true).open(&self.path).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(|line| line.unwrap())
    }
}
