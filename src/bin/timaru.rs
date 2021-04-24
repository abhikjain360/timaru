use chrono::{Datelike, Duration, Local, TimeZone};

use timaru::{
    cli::{Opts, PomodoroUpdate, SubCommand, UpdateSubCmd},
    parser::get_ymd,
    schedule::Schedule,
    setup::check_setup,
    task::{Task, TaskTime},
};

use clap::Clap;

fn main() {
    let (_cfg_dir, db_dir) = check_setup().unwrap();

    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Week => {
            let mut day = Local::today();
            for _ in 0..7 {
                println!("{:?}", Schedule::open(&db_dir, &day).unwrap());
                day = day + Duration::days(1);
            }
        }
        SubCommand::Month => {
            let mut day = Local::today();
            let next_month = if day.month() < 12 { day.month() + 1 } else { 1 };
            let next_month_day = Local.ymd(day.year(), next_month, day.day());
            while day <= next_month_day {
                println!("{:?}", Schedule::open(&db_dir, &day).unwrap());
                day = day + Duration::days(1);
            }
        }
        SubCommand::List { date } => {
            let date = match date {
                Some(date_string) => {
                    let (_, (d, m, y)) = get_ymd(&date_string).unwrap();
                    Local.ymd(y, m, d)
                }
                None => Local::today(),
            };
            println!("{:?}", Schedule::open(&db_dir, &date).unwrap());
        }
        SubCommand::Add {
            date,
            time,
            pomodoro,
            description,
        } => {
            let date = match date {
                Some(date_string) => {
                    let (_, (d, m, y)) = get_ymd(&date_string).unwrap();
                    Local.ymd(y, m, d)
                }
                None => Local::today(),
            };

            let task = Task {
                time: match time {
                    Some(time) => TaskTime::from_str(&time, &date).unwrap(),
                    None => TaskTime::Precise { time: Local::now() },
                },
                description,
                pomodoro: pomodoro.map(|total| (total, 0)),
                finished: false,
            };

            let mut schedule = Schedule::open(&db_dir, &date).unwrap();
            schedule.add_task(task);
        }
        SubCommand::Remove { date, idx } => {
            let date = {
                let (_, (d, m, y)) = get_ymd(&date).unwrap();
                Local.ymd(y, m, d)
            };

            let mut schedule = Schedule::open(&db_dir, &date).unwrap();
            schedule.remove_task(idx).unwrap();
            // println!("{}\n{:?}", schedule.as_string(), schedule);
        }
        #[allow(unused_variables)]
        SubCommand::Update {
            old_date,
            idx,
            subcmd,
        } => {
            let old_date = {
                let (_, (d, m, y)) = get_ymd(&old_date).unwrap();
                Local.ymd(y, m, d)
            };
            let mut old_task_schedule = Schedule::open(&db_dir, &old_date).unwrap();

            match subcmd {
                UpdateSubCmd::Date { date } => {
                    let date = {
                        let (_, (d, m, y)) = get_ymd(&date).unwrap();
                        Local.ymd(y, m, d)
                    };
                    let mut task = old_task_schedule.remove_task(idx).unwrap();
                    task.time.change_date(&date);
                    let mut new_schedule = Schedule::open(&db_dir, &date).unwrap();
                    new_schedule.add_task(task);
                }
                UpdateSubCmd::Time { time } => {
                    old_task_schedule.tasks.get_mut(&idx).unwrap().time =
                        TaskTime::from_str(&time, &old_task_schedule.date).unwrap();
                }
                UpdateSubCmd::Description { desc } => {
                    old_task_schedule.tasks.get_mut(&idx).unwrap().description = desc;
                }
                UpdateSubCmd::Pomodoro(pom_update) => match pom_update {
                    PomodoroUpdate::New { total } => {
                        let task = old_task_schedule.tasks.get_mut(&idx).unwrap();
                        task.pomodoro = match &task.pomodoro {
                            Some((_, done)) => Some((total, *done)),
                            None => Some((total, 0)),
                        };
                    }
                    PomodoroUpdate::Done { done } => {
                        let task = old_task_schedule.tasks.get_mut(&idx).unwrap();
                        task.pomodoro = Some((task.pomodoro.unwrap().0, done));
                    }
                    PomodoroUpdate::Remove => {
                        old_task_schedule.tasks.get_mut(&idx).unwrap().pomodoro = None;
                    }
                },
                UpdateSubCmd::Done => {
                    old_task_schedule.tasks.get_mut(&idx).unwrap().finished = true;
                }
                UpdateSubCmd::NotDone => {
                    old_task_schedule.tasks.get_mut(&idx).unwrap().finished = false;
                }
            }
        }
    }
}
