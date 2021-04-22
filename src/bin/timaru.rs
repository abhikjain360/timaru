use chrono::{Datelike, Duration, Local, TimeZone};

use timaru::{
    cli::{Opts, SubCommand},
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
        SubCommand::Today => {
            println!("{:?}", Schedule::open(&db_dir, &Local::today()).unwrap());
        }
        SubCommand::Weekly => {
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
        SubCommand::List { date: _ } => {
            println!("list");
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
        } => {}
    }
}
