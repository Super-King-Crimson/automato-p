use crate::schedule::Schedule;

pub struct ScheduleList {
    list: Vec<Schedule>,
}

impl ScheduleList {
    pub fn default() -> ScheduleList {
        ScheduleList { list: vec![Schedule::pomodoro()] }
    }

    pub fn from(schedules: Vec<Schedule>) -> ScheduleList {
        ScheduleList { list: schedules }
    }

    pub fn start_schedule(&self, index: usize, alarm_path: Option<&str>) {
        self.get(index).start(alarm_path);
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
    
    pub fn display_list(&self) {
        self.list.iter().enumerate().for_each(|(i, sch)| println!("{i}: {sch}"));
    }
    
    pub fn get(&self, index: usize) -> &Schedule {
        self.list.get(index).unwrap()
    }

    pub fn push(&mut self, schedule: Schedule) {
        self.list.push(schedule);
    }

    pub fn insert(&mut self, index: usize, schedule: Schedule) {
        self.list.insert(index, schedule);
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn replace(&mut self, index: usize, replacement: Schedule) {
        self.list[index] = replacement;
    }
}