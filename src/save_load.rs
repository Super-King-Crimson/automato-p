use std::{fs::{self, File, OpenOptions}, io::{Error, Write, BufWriter, BufRead, BufReader}, path::Path};
use crate::schedule::Schedule;

pub struct SaveLoad {
    path: Path
}

impl SaveLoad {
    fn read_all_schedules(&self) {
        let file = OpenOptions::new().read(true).open(self.path).unwrap();
    }

    pub fn try_new(path: Path) -> Result<SaveLoad, Error> {
        match path.try_exists() {
            Ok(_) => Ok(SaveLoad { path }),
            Err(e) => e,
        }
    }
}

fn read_schedules_from_file(path: SaveLoad) -> Vec<Schedule> {
    let file = fs::File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| serde_json::from_str(&line.unwrap()).unwrap()).collect()
}

fn write_schedule_to_file(data: &Schedule, path: SaveLoadPath) {
    let mut json = serde_json::to_string(data).unwrap();
    json.push('\n');

    let file = fs::OpenOptions::new().write(true).append(true).open(SCHEDULE_FILE).unwrap();
    let mut file = BufWriter::new(file);

    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}

pub fn delete_schedule(index: usize, path: SaveLoadPath) {
    let file = fs::OpenOptions::new().read(true).write(true).open(path).unwrap();
    let reader = BufReader::new(&file);
    
    let mut new_contents = String::new();

    for (i, line) in reader.lines().enumerate() {
        if i != index {
            new_contents.push_str(&line.unwrap());
        }
    }

    let mut writer = BufWriter::new(&file);
    writer.write_all(new_contents.as_bytes()).unwrap();
    writer.flush().unwrap();
}

pub fn replace_schedule(index: usize, replacement: &Schedule, path: SaveLoadPath) {
    let file = fs::OpenOptions::new().read(true).write(true).open(PATH_TO_FILE).unwrap();
    let reader = BufReader::new(&file);
    
    let mut new_contents = String::new();

    for (i, line) in reader.lines().enumerate() {
        if i != index {
            new_contents.push_str(&line.unwrap());
        } else {
            new_contents.push_str(&serde_json::to_string(replacement).unwrap())
        }
    }

    let mut writer = BufWriter::new(&file);
    writer.write_all(new_contents.as_bytes()).unwrap();
    writer.flush().unwrap();
}

pub fn save_schedule(schedule: &Schedule) {
    write_schedule_to_file(schedule);
}

pub fn load_schedules() -> Vec<Schedule> {
    read_schedules_from_file()
}
