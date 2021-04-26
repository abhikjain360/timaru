use chrono::{Datelike, Duration, Local, TimeZone};

use timaru::{
    cli::{Opts, PomodoroUpdate, SubCommand, UpdateSubCmd},
    parser::get_ymd,
    schedule::Schedule,
    setup::check_setup,
    task::{Task, TaskTime},
};

use clap::Clap;

fn run() -> Result<(), String> {
    let (_cfg_dir, db_dir) = match check_setup() {
        Ok(path_tuple) => path_tuple,
        Err(err) => {
            return Err(format!("error: could not read configuration directory {:?}", err))
        }
    };

    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Week => {
            let mut day = Local::today();
            for _ in 0..7 {
                match Schedule::open(&db_dir, &day) {
                    Ok(schedule ) => println!("{:?}", schedule),
                    Err(err) => return Err(format!("error: could not open schedule {:?}", err))
                }
                day = day + Duration::days(1);
            }
        }
        SubCommand::Month => {
            let mut day = Local::today();
            let next_month = if day.month() < 12 { day.month() + 1 } else { 1 };
            let next_month_day = Local.ymd(day.year(), next_month, day.day());
            while day <= next_month_day {
                match Schedule::open(&db_dir, &day) {
                    Ok(schedule ) => println!("{:?}", schedule),
                    Err(err) => return Err(format!("error: could not open schedule {:?}", err))
                }
                day = day + Duration::days(1);
            }
        }
        SubCommand::List { date } => {
            let date = match date {
                Some(date_string) => {
                    let (d, m, y) = match get_ymd(&date_string) {
                        Ok((_, date)) => date,
                        Err(err) => return Err(format!("error: could not open schedule {}", err)),
                    };
                    Local.ymd(y, m, d)
                }
                None => Local::today(),
            };
            match Schedule::open(&db_dir, &date) {
                Ok(schedule ) => println!("{:?}", schedule),
                Err(err) => return Err(format!("error: could not open schedule {:?}", err)),
            }
        }
        SubCommand::Add {
            date,
            time,
            pomodoro,
            description,
        } => {
            let date = match date {
                Some(date_string) => {
                    let (d, m, y) = match get_ymd(&date_string) {
                        Ok((_, date)) => date,
                        Err(err) => return Err(format!("error: invalid date {}", err)),
                    };
                    Local.ymd(y, m, d)
                }
                None => Local::today(),
            };

            let task = Task {
                time: match time {
                    Some(time) => match TaskTime::from_str(&time, &date) {
                        Ok(time) => time,
                        Err(err) => return Err(format!("error: invalid time {:?}", err)),
                    }
                    None => TaskTime::Precise { time: Local::now() },
                },
                description,
                pomodoro: pomodoro.map(|total| (total, 0)),
                finished: false,
            };
            match Schedule::open(&db_dir, &date) {
                Ok(ref mut schedule ) => schedule.add_task(task),
                Err(err) => return Err(format!("error: could not open schedule {:?}", err)),
            }
        }
        SubCommand::Remove { date, idx } => {
            let date = {
                let (d, m, y) = match get_ymd(&date) {
                    Ok((_, date)) => date,
                    Err(err) => return Err(format!("error: invalid date {}", err)),
                };
                Local.ymd(y, m, d)
            };

            match Schedule::open(&db_dir, &date) {
                Ok(ref mut schedule ) => {
                    if let None = schedule.remove_task(idx) {
                        return Err(format!("error: invalid index"))
                    }
                    // println!("{}\n{:?}", schedule.as_string(), schedule);
                }
                Err(err) => return Err(format!("error: could not open schedule {:?}", err)),
            }
        }
        #[allow(unused_variables)]
        SubCommand::Update {
            old_date,
            idx,
            subcmd,
        } => {
            let old_date = {
                let (d, m, y) = match get_ymd(&old_date) {
                    Ok((_, date)) => date,
                    Err(err) => return Err(format!("error: invalid date {}", err)),
                };
                Local.ymd(y, m, d)
            };

            let mut old_task_schedule = match Schedule::open(&db_dir, &old_date) {
                Ok(schedule) => schedule,
                Err(err) => return Err(format!("error: could not open schedule {:?}", err)),
            };

            match subcmd {
                UpdateSubCmd::Date { date } => {
                    let date = {
                        let (d, m, y) = match get_ymd(&date) {
                            Ok((_, date)) => date,
                            Err(err) => return Err(format!("error: invalid date {}", err)),
                        };
                        Local.ymd(y, m, d)
                    };
                    if let Some(mut task) = old_task_schedule.remove_task(idx) {
                        task.time.change_date(&date);
                        match Schedule::open(&db_dir, &date) {
                            Ok(mut new_schedule ) => new_schedule.add_task(task),
                            Err(err) => return Err(format!("error: could not open schedule {:?}", err)),
                        }
                    } else {
                        return Err(format!("error: invalid index"))
                    }
                }
                UpdateSubCmd::Time { time } => {
                    if let Some(task) = old_task_schedule.tasks.get_mut(&idx) {
                        if let Err(err) = TaskTime::from_str(&time, &old_task_schedule.date) {
                            return Err(format!("error: invalid time {:?}", err))
                        }
                    } else {
                        return Err(format!("error: invalid index"))
                    }                        
                }
                UpdateSubCmd::Description { desc } => {
                    if let Some(task) = old_task_schedule.tasks.get_mut(&idx) {
                        task.description = desc;
                    } else {
                        return Err(format!("error: invalid index"))
                    }
                }
                UpdateSubCmd::Pomodoro(pom_update) => match pom_update {
                    PomodoroUpdate::New { total } => {
                        if let Some(task) = old_task_schedule.tasks.get_mut(&idx) {
                            task.pomodoro = match &task.pomodoro {
                                Some((_, done)) => Some((total, *done)),
                                None => Some((total, 0)),
                            };
                        } else {
                            return Err(format!("error: invalid index"))
                        }
                    }
                    PomodoroUpdate::Done { done } => {
                        if let Some(task) = old_task_schedule.tasks.get_mut(&idx) {
                            task.pomodoro = Some((task.pomodoro.unwrap().0, done)); // ?
                        } else {
                            return Err(format!("error: invalid index"))
                        }
                    }
                    PomodoroUpdate::Remove => {
                        if let Some(task) = old_task_schedule.tasks.get_mut(&idx) {
                            task.pomodoro = None;
                        } else {
                            return Err(format!("error: invalid index"))
                        }
                    }
                },
                UpdateSubCmd::Done => {
                    if let Some(task) = old_task_schedule.tasks.get_mut(&idx) {
                        task.finished = true;
                    } else {
                        return Err(format!("error: invalid index"))
                    }
                }
                UpdateSubCmd::NotDone => {
                    if let Some(task) = old_task_schedule.tasks.get_mut(&idx) {
                        task.finished = false;
                    } else {
                        return Err(format!("error: invalid index"))
                    }
                }
            }
        }
    }
    Ok(())
}
fn main() {
    if let Err(err) = run() {
        eprintln!("{}" , err);
    }
}
