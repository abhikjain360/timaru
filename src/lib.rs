use std::{collections::HashMap, path::PathBuf};

use chrono::{Date, DateTime, Local};

pub mod cli;
pub mod error;
pub mod database;

#[derive(Debug, Clone)]
pub struct Schedule {
    pub date: Date<Local>,
    pub tasks: HashMap<u8, Task>,
    pub file: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub time: TaskTime,
    pub description: String,
    pub pomodoro: Option<(u8, u8)>,
    pub finished: bool,
}

#[derive(Debug, Clone)]
pub enum TaskTime {
    Precise {
        time: DateTime<Local>,
    },
    General {
        time: TimeOfDay,
    },
    Period {
        start: DateTime<Local>,
        end: DateTime<Local>,
    },
    GeneralPeriod {
        start: TimeOfDay,
        end: TimeOfDay,
    },
}

#[derive(Debug, Clone)]
pub enum TimeOfDay {
    Morning,
    Noon,
    AfterNoon,
    Evening,
    Night,
    MidNight,
    Custom(String),
}
