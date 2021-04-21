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
            .map_err(|_| TimaruError::File(schedule_path))?;

        Schedule::from_str(&schedule_content)
    }

    pub fn close(self, db_dir: &PathBuf) -> Result<(), TimaruError> {
        let schedule_path = check_dir(
            check_dir(db_dir.join(&format!("{}", self.date.year())))?
                .join(&format!("{}", self.date.month())),
        )?
        .join(&format!("{}", self.date.day()));

        let mut schedule_file = BufWriter::new(
            match OpenOptions::new()
                .create(true)
                .write(true)
                .open(&schedule_path)
            {
                Ok(file) => file,
                Err(_) => return Err(TimaruError::File(schedule_path)),
            },
        );

        match schedule_file.write(self.as_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(TimaruError::File(schedule_path)),
        }
    }
}
