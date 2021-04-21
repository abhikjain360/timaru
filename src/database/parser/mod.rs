use std::{
    fs::OpenOptions,
    io::{prelude::*, BufReader, BufWriter},
    path::PathBuf,
};

use chrono::{Date, Datelike, Local};

use crate::{
    database::{setup::check_dir, Schedule},
    error::TimaruError,
};

mod from_string;
mod to_string;

pub use from_string::*;
pub use to_string::*;

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
            .map_err(|e| {
                println!("{:?}", e);
                TimaruError::File(schedule_path.clone())
            })?;

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
            match OpenOptions::new().create(true).write(true).open(&self.file) {
                Ok(file) => file,
                Err(_) => return Err(TimaruError::File(self.file.clone())),
            },
        );

        match schedule_file.write(self.as_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(TimaruError::File(self.file.clone())),
        }
    }
}

impl Drop for Schedule {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}
