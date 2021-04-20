use std::collections::HashMap;

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

use super::{Schedule, Task, TaskTime, TimeOfDay};

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

fn clear_ws(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| is_space(c as u8) || is_newline(c as u8))(input)
}

fn parse_time(input: &str, date: &Date<Local>) -> Result<DateTime<Local>, ()> {
    let s: Vec<&str> = input.split(":").collect();
    match s.len() {
        1 => match s[0].parse::<u32>() {
            Ok(hour) => Ok(date.and_hms(hour, 0, 0)),
            _ => Err(()),
        },
        2 => match (s[0].parse::<u32>(), s[1].parse::<u32>()) {
            (Ok(hour), Ok(min)) => Ok(date.and_hms(hour, min, 0)),
            _ => Err(()),
        },
        3 => match (
            s[0].parse::<u32>(),
            s[1].parse::<u32>(),
            s[2].parse::<u32>(),
        ) {
            (Ok(hour), Ok(min), Ok(sec)) => Ok(date.and_hms(hour, min, sec)),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

impl Schedule {
    pub fn from_str(input: &str) -> IResult<&str, Self> {
        let (input, (_, _, _, day, month, year)) =
            tuple((clear_ws, char('#'), space0, get_day, get_month, get_year))(input)?;

        let date = match Local.ymd_opt(year, month, day) {
            LocalResult::None => {
                return Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Digit,
                }))
            }
            LocalResult::Single(date) => date,
            LocalResult::Ambiguous(date, _) => date,
        };

        let mut tasks = HashMap::with_capacity(5);

        let (input, _) = clear_ws(input)?;

        for (idx, line) in input.lines().enumerate() {
            tasks.insert(idx as u8, Task::from_str(line, &date)?.1);
        }

        Ok((input, Self { date, tasks }))
    }
}

impl Task {
    pub fn from_str<'s, 'd>(input: &'s str, date: &'d Date<Local>) -> IResult<&'s str, Self> {
        let (mut input, _) =
            tuple((space0, alt((char('-'), char('*'))), space0, char('[')))(input)?;

        let finished =
            if let Ok((input_left, _)) = char::<&str, nom::error::Error<&str>>('X')(input) {
                input = input_left;
                true
            } else if let Ok((input_left, _)) = space0::<&str, nom::error::Error<&str>>(input) {
                input = input_left;
                false
            } else {
                return Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Space,
                }));
            };

        let (input, (_, _, time_str, _)) = tuple((
            char(']'),
            space1,
            alt((take_until("("), take_until("=>"))),
            space0,
        ))(input)?;

        let time = TaskTime::from_str(time_str.trim(), date).map_err(|_| {
            nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Space,
            })
        })?;

        let (description, pomodoro) = if input.starts_with("(") {
            let (input, (_, _, times, _, _, _, done, _, _, _, _, _)) = tuple((
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
            ))(input)?;
            match (times.parse::<u8>(), done.parse::<u8>()) {
                (Ok(times), Ok(done)) => (input, Some((times, done))),
                _ => {
                    return Err(nom::Err::Error(nom::error::Error {
                        input,
                        code: nom::error::ErrorKind::Space,
                    }))
                }
            }
        } else {
            (tuple((space0, tag("=>"), space0))(input)?.0, None)
        };

        Ok((
            "",
            Self {
                time,
                pomodoro,
                description: description.to_string(),
                finished,
            },
        ))
    }
}

impl TaskTime {
    pub fn from_str(input: &str, date: &Date<Local>) -> Result<TaskTime, ()> {
        let s: Vec<&str> = input.split('-').collect();

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
            _ => Err(()),
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
    #[test]
    fn test_schedule_parsing() {
        use super::Schedule;

        let schedule_str = r#"
# 12-12-2012

* [ ] 4:30 (1, 0) => do some stuff
- [X] 5:30 (1, 1) => do some other stuff
"#;

        let (_, schedule) = Schedule::from_str(schedule_str).unwrap();
        println!("{:?}", schedule);
    }
}
