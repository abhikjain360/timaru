use chrono::{Datelike, Duration, Local, TimeZone};

use timaru::{
    cli::{Opts, Remove, SubCommand},
    parser::get_ymd,
    schedule::Schedule,
    setup::check_setup,
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
        SubCommand::Add(add) => {
            Schedule::from_add(add, &db_dir).unwrap();
        }
        SubCommand::Remove(Remove { date, idx }) => {
            let date = {
                let (_, (d, m, y)) = get_ymd(&date).unwrap();
                Local.ymd(y, m, d)
            };

            let mut schedule = Schedule::open(&db_dir, &date).unwrap();
            schedule.remove_task(idx).unwrap();
            // println!("{}\n{:?}", schedule.as_string(), schedule);
        }
        SubCommand::Update(_update) => {
            println!("update");
        }
    }
}
