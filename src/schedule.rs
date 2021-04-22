use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    fs::OpenOptions,
    io::{prelude::*, BufReader, BufWriter},
    path::PathBuf,
};

use chrono::{Date, Datelike, Local, TimeZone};

use crate::{
    change_err,
    cli::Add,
    error::TimaruError,
    parser::get_ymd,
    setup::check_dir,
    task::{Task, TaskTime},
};

#[derive(Clone)]
pub struct Schedule {
    pub date: Date<Local>,
    pub tasks: HashMap<u8, Task>,
    pub file: PathBuf,
}

impl Debug for Schedule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let mut s = format!(
            "# {}-{}-{}\n",
            self.date.day(),
            self.date.month(),
            self.date.year()
        );

        for (idx, task) in self.tasks.iter() {
            s += &format!("({}) {}\n", idx, task.as_string());
        }

        f.write_str(&s)
    }
}

impl Schedule {
    pub fn open(db_dir: &PathBuf, date: &Date<Local>) -> Result<Schedule, TimaruError> {
        let schedule_path = check_dir(
            check_dir(db_dir.join(&format!("{}", date.year())))?.join(&format!("{}", date.month())),
        )?
        .join(&format!("{}", date.day()));

        let mut schedule_file = BufReader::new(
            match OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&schedule_path)
            {
                Ok(file) => file,
                Err(_) => return Err(TimaruError::File(schedule_path)),
            },
        );

        let mut schedule_content = String::new();
        schedule_file
            .read_to_string(&mut schedule_content)
            .map_err(|_| TimaruError::File(schedule_path.clone()))?;

        let schedule_content = schedule_content.trim();

        if schedule_content.is_empty() {
            Ok(Schedule {
                file: schedule_path,
                tasks: Default::default(),
                date: *date,
            })
        } else {
            Schedule::from_str(schedule_path, &schedule_content)
        }
    }

    pub fn flush(&self) -> Result<(), TimaruError> {
        let mut schedule_file = BufWriter::new(
            match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.file)
            {
                Ok(file) => file,
                Err(_) => return Err(TimaruError::File(self.file.clone())),
            },
        );

        match schedule_file.write_all(self.as_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(TimaruError::File(self.file.clone())),
        }
    }

    pub fn from_add(
        Add {
            date,
            time,
            pomodoro,
            description,
        }: Add,
        db_dir: &PathBuf,
    ) -> Result<Self, TimaruError> {
        let date = match date {
            Some(date_string) => {
                let (_, (d, m, y)) = change_err!(get_ymd(&date_string), "date");
                Local.ymd(y, m, d)
            }
            None => Local::today(),
        };

        let task = Task {
            time: match time {
                Some(time) => TaskTime::from_str(&time, &date)?,
                None => TaskTime::Precise { time: Local::now() },
            },
            description,
            pomodoro: pomodoro.map(|total| (total, 0)),
            finished: false,
        };

        let mut schedule = Schedule::open(db_dir, &date)?;
        schedule.add_task(task);

        Ok(schedule)
    }

    pub fn add_task(&mut self, task: Task) {
        let idx = self.tasks.len() as u8 + 1;
        self.tasks.insert(idx, task);
    }

    pub fn remove_task(&mut self, idx: u8) -> Option<Task> {
        self.tasks.remove(&idx)
    }
}

impl Drop for Schedule {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}
