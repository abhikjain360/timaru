#![allow(dead_code)]

use std::collections::HashMap;

use chrono::{Date, DateTime, Local};

pub mod parser;
pub mod fs;

#[derive(Debug, Clone)]
pub struct Schedule {
    pub date: Date<Local>,
    pub tasks: HashMap<u8, Task>,
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
