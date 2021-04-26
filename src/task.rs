use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use chrono::{Date, DateTime, Local, Timelike, NaiveTime};

#[derive(Debug, Clone)]
pub struct Task {
    pub time: TaskTime,
    pub description: String,
    pub pomodoro: Option<(u8, u8)>,
    pub finished: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimeOfDay {
    Morning,
    Noon,
    AfterNoon,
    Evening,
    Night,
    MidNight,
    Custom(String),
}

impl TaskTime {
    pub fn change_date(&mut self, date: &Date<Local>) {
        match self {
            TaskTime::Period { start, end } => {
                *start = date.and_hms(start.hour(), start.minute(), start.second());
                *end = date.and_hms(end.hour(), end.minute(), end.second());
            }
            TaskTime::Precise { time } => {
                *time = date.and_hms(time.hour(), time.minute(), time.second());
            }
            _ => {}
        }
    }
}

impl PartialOrd for TaskTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        None
    }
}

impl Ord for TaskTime {
    fn cmp(&self, other: &Self) -> Ordering {

    }
}

impl TimeOfDay {
    pub fn to_time(&self) -> Option<NaiveTime> {
        match self {
            &TimeOfDay::Noon => Some(NaiveTime::from_hms(12, 0, 0)),
            _ => None
        }
    }
}
