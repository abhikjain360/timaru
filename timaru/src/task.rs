use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use chrono::{Date, DateTime, Local, NaiveTime, Timelike};

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

// TODO: make all the `TimeOfDay` variants hold a user-provided time/time-range, along with a
// `Default` implementation.
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
        let mut other_end_time = None;

        let other_time = match other {
            TaskTime::Precise { time } => time.time(),
            TaskTime::General { time } => {
                let res = time.to_time()?;
                other_end_time = res.1;
                res.0
            }
            TaskTime::Period { start, end } => {
                other_end_time = Some(end.time());
                start.time()
            }
            TaskTime::GeneralPeriod { start, end } => {
                other_end_time = Some(match end.to_time()? {
                    (start, None) => start,
                    (_, Some(end)) => end,
                });
                start.to_time()?.0
            }
        };

        match self {
            TaskTime::Precise { time } => {
                let self_time = time.time();
                match other_end_time {
                    Some(other_end_time) => {
                        if self_time < other_time {
                            Some(Ordering::Less)
                        } else if self_time < other_end_time {
                            Some(Ordering::Equal)
                        } else {
                            Some(Ordering::Greater)
                        }
                    }
                    None => self_time.partial_cmp(&other_time),
                }
            }
            TaskTime::General { time } => match other_end_time {
                Some(other_end_time) => match time.to_time()? {
                    (self_time, None) => {
                        if self_time < other_time {
                            Some(Ordering::Less)
                        } else if self_time <= other_end_time {
                            Some(Ordering::Equal)
                        } else {
                            Some(Ordering::Greater)
                        }
                    }
                    // FIXME: maybe there is a more logical way to compare 2 ranges
                    (start, _) => start.partial_cmp(&other_time),
                },
                None => Some(time.to_time()?.0.cmp(&other_time)),
            },
            TaskTime::Period { start, end } => match other_end_time {
                // FIXME: maybe there is a more logical way to compare 2 ranges
                Some(_) => Some(start.time().cmp(&other_time)),
                None => {
                    if other_time < start.time() {
                        Some(Ordering::Greater)
                    } else if other_time <= end.time() {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Less)
                    }
                }
            },
            TaskTime::GeneralPeriod { start, end } => match other_end_time {
                Some(_) => Some(start.to_time()?.0.cmp(&other_time)),
                None => {
                    if other_time < start.to_time()?.0 {
                        Some(Ordering::Greater)
                    } else if other_time <= end.to_time()?.0 {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Less)
                    }
                }
            },
        }
    }
}

impl TimeOfDay {
    pub fn to_time(&self) -> Option<(NaiveTime, Option<NaiveTime>)> {
        match self {
            TimeOfDay::Morning => Some((
                NaiveTime::from_hms(6, 0, 0),
                Some(NaiveTime::from_hms(11, 59, 59)),
            )),
            TimeOfDay::Noon => Some((NaiveTime::from_hms(12, 0, 0), None)),
            TimeOfDay::AfterNoon => Some((
                NaiveTime::from_hms(12, 0, 1),
                Some(NaiveTime::from_hms(17, 0, 0)),
            )),
            TimeOfDay::Evening => Some((
                NaiveTime::from_hms(17, 0, 1),
                Some(NaiveTime::from_hms(20, 0, 0)),
            )),
            TimeOfDay::Night => Some((
                NaiveTime::from_hms(20, 0, 1),
                Some(NaiveTime::from_hms(23, 59, 59)),
            )),
            TimeOfDay::MidNight => Some((NaiveTime::from_hms(0, 0, 0), None)),
            _ => None,
        }
    }
}
