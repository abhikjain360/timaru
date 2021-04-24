use chrono::{Datelike, Timelike};

use crate::{
    schedule::Schedule,
    task::{Task, TaskTime, TimeOfDay},
};

impl Schedule {
    pub fn as_string(&self) -> String {
        let mut s = format!(
            "# {}-{}-{}\n",
            self.date.day(),
            self.date.month(),
            self.date.year()
        );

        for task in self.tasks.values() {
            s += &format!("{}\n", task.as_string());
        }

        s
    }
}

impl Task {
    pub fn as_string(&self) -> String {
        let mut s = format!(
            "* [{}] {} ",
            if self.finished { 'X' } else { ' ' },
            self.time.as_string()
        );
        if let Some((total, done)) = self.pomodoro {
            s += &format!("({}, {}) ", total, done);
        }
        s += &format!("=> {}", self.description);
        s
    }
}

impl TaskTime {
    pub fn as_string(&self) -> String {
        match self {
            TaskTime::Precise { time } => {
                if time.minute() < 10 {
                    format!("{}:0{}", time.hour(), time.minute())
                } else {
                    format!("{}:{}", time.hour(), time.minute())
                }
            }
            TaskTime::General { time } => time.as_str().to_string(),
            TaskTime::Period { start, end } => format!(
                "{}:{} - {}:{}",
                start.hour(),
                start.minute(),
                end.hour(),
                end.minute()
            ),
            TaskTime::GeneralPeriod { start, end } => {
                format!("{} - {}", start.as_str(), end.as_str())
            }
        }
    }
}

impl TimeOfDay {
    pub fn as_str(&self) -> &str {
        match self {
            TimeOfDay::Morning => "morning",
            TimeOfDay::Noon => "noon",
            TimeOfDay::AfterNoon => "afternoon",
            TimeOfDay::Evening => "evening",
            TimeOfDay::Night => "night",
            TimeOfDay::MidNight => "midnight",
            TimeOfDay::Custom(input) => input,
        }
    }
}
