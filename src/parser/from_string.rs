use std::{collections::HashMap, path::PathBuf};

use chrono::{Date, DateTime, Local, LocalResult, TimeZone};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::{
        complete::{char, digit1, space0, space1},
        is_newline, is_space,
    },
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use crate::{
    change_err,
    error::TimaruError,
    schedule::Schedule,
    task::{Task, TaskTime, TimeOfDay},
};

fn get_day(input: &str) -> IResult<&str, u32> {
    let (input, day) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (left_str, _) = char('-')(input)?;
    Ok((left_str, day))
}

fn get_month(input: &str) -> IResult<&str, u32> {
    let (input, month) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (left_str, _) = char('-')(input)?;
    Ok((left_str, month))
}

fn get_year(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

pub fn get_ymd(input: &str) -> IResult<&str, (u32, u32, i32)> {
    tuple((space0, get_day, get_month, get_year))(input)
        .map(|t| (t.0, (t.1 .1, t.1 .2, t.1 .3)))
        .map_err(|e| {
            println!("{:?}", e);
            e
        })
}

fn clear_ws(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| is_space(c as u8) || is_newline(c as u8))(input)
}

pub fn parse_time(input: &str, date: &Date<Local>) -> Result<DateTime<Local>, TimaruError> {
    let s: Vec<&str> = input.split(':').collect();
    match s.len() {
        1 => match s[0].parse::<u32>() {
            Ok(hour) => Ok(date.and_hms(hour, 0, 0)),
            _ => Err(TimaruError::Parse("time")),
        },
        2 => match (s[0].parse::<u32>(), s[1].parse::<u32>()) {
            (Ok(hour), Ok(min)) => Ok(date.and_hms(hour, min, 0)),
            _ => Err(TimaruError::Parse("time")),
        },
        3 => match (
            s[0].parse::<u32>(),
            s[1].parse::<u32>(),
            s[2].parse::<u32>(),
        ) {
            (Ok(hour), Ok(min), Ok(sec)) => Ok(date.and_hms(hour, min, sec)),
            _ => Err(TimaruError::Parse("time")),
        },
        _ => Err(TimaruError::Parse("time")),
    }
}

impl Schedule {
    pub fn from_str(file: PathBuf, input: &str) -> Result<Self, TimaruError> {
        let (input, (_, _, _, day, month, year)) = change_err!(
            tuple((clear_ws, char('#'), space0, get_day, get_month, get_year))(input),
            "date format wrong"
        );

        let date = match Local.ymd_opt(year, month, day) {
            LocalResult::None => return Err(TimaruError::Parse("date can not exist")),
            LocalResult::Single(date) => date,
            LocalResult::Ambiguous(date, _) => date,
        };

        let mut tasks = HashMap::with_capacity(5);

        let (input, _) = change_err!(clear_ws(input), "whitespace before tasks");

        for (idx, line) in input.lines().enumerate() {
            tasks.insert((idx as u8) + 1, Task::from_str(line, &date)?);
        }

        Ok(Self { date, tasks, file })
    }
}

impl Task {
    pub fn from_str(input: &'_ str, date: &Date<Local>) -> Result<Self, TimaruError> {
        let (mut input, _) = change_err!(
            tuple::<&str, _, nom::error::Error<&str>, _>((
                space0,
                alt((char('-'), char('*'))),
                space0,
                char('[')
            ))(input),
            "start of task"
        );

        let finished =
            if let Ok((input_left, _)) = char::<&str, nom::error::Error<&str>>('X')(input) {
                input = input_left;
                true
            } else if let Ok((input_left, _)) = space0::<&str, nom::error::Error<&str>>(input) {
                input = input_left;
                false
            } else {
                return Err(TimaruError::Parse("finished marking of task"));
            };

        let (input, (_, _, time_str, _)) = change_err!(
            tuple::<&str, _, nom::error::Error<&str>, _>((
                char(']'),
                space1,
                alt((take_until("("), take_until("=>"))),
                space0,
            ))(input),
            "task time"
        );

        let time = TaskTime::from_str(time_str.trim(), date)?;

        let (description, pomodoro) = if input.starts_with('(') {
            let (input, (_, _, times, _, _, _, done, _, _, _, _, _)) = change_err!(
                tuple::<&str, _, nom::error::Error<&str>, _>((
                    char('('),
                    space0,
                    digit1,
                    space0,
                    char(','),
                    space0,
                    digit1,
                    space0,
                    char(')'),
                    space0,
                    tag("=>"),
                    space0,
                ))(input),
                "pomodoro and/or description"
            );
            match (times.parse::<u8>(), done.parse::<u8>()) {
                (Ok(times), Ok(done)) => (input, Some((times, done))),
                _ => return Err(TimaruError::Parse("pomodoro and/or description")),
            }
        } else {
            (
                change_err!(
                    tuple::<&str, _, nom::error::Error<&str>, _>((space0, tag("=>"), space0))(
                        input
                    ),
                    "description"
                )
                .0,
                None,
            )
        };

        Ok(Self {
            time,
            pomodoro,
            description: description.to_string(),
            finished,
        })
    }
}

impl TaskTime {
    pub fn from_str(input: &str, date: &Date<Local>) -> Result<TaskTime, TimaruError> {
        let s: Vec<&str> = input.split('-').map(|s| s.trim()).collect();

        match s.len() {
            1 => {
                if let Ok(time) = parse_time(s[0], date) {
                    Ok(TaskTime::Precise { time })
                } else {
                    Ok(TaskTime::General {
                        time: TimeOfDay::from(s[0]),
                    })
                }
            }

            2 => {
                if let Ok(start) = parse_time(s[0], date) {
                    parse_time(s[1], date).map(|end| TaskTime::Period { start, end })
                } else {
                    Ok(TaskTime::GeneralPeriod {
                        start: TimeOfDay::from(s[0]),
                        end: TimeOfDay::from(s[1]),
                    })
                }
            }
            _ => Err(TimaruError::Parse("task time")),
        }
    }
}

impl From<&str> for TimeOfDay {
    #[rustfmt::skip]
    fn from(input: &str) -> Self {
        let input_lowercase = input.to_lowercase();
        match input_lowercase.as_str() {
            "morning"   => TimeOfDay::Morning,
            "noon"      => TimeOfDay::Noon,
            "afternoon" => TimeOfDay::AfterNoon,
            "evening"   => TimeOfDay::Evening,
            "night"     => TimeOfDay::Night,
            "midnight"  => TimeOfDay::MidNight,
            _           => TimeOfDay::Custom(input.into()),
        }
    }
}

mod test {

    // TODO: fix tests here by adding more and using assertions.
    #[test]
    fn test_schedule_parsing() {
        use super::Schedule;
        use std::path::PathBuf;

        let schedule_str = r#"
# 12-12-2012

* [ ] 4:30 (1, 0) => do some stuff
- [X] 5:30 (1, 1) => do some other stuff
"#;

        let schedule = Schedule::from_str(PathBuf::from(""), schedule_str).unwrap();
        println!("{:?}", schedule);
    }
}
