#![allow(unused_imports)]

use chrono::{Date, Datelike, Duration, Local, TimeZone};

use timaru::{
    cli::{Opts, SubCommand},
    database::setup::check_setup,
    Schedule,
};

use clap::Clap;

fn main() {
    let (_cfg_dir, db_dir) = check_setup().unwrap();

    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Today => {
            println!(
                "{}",
                Schedule::open(&db_dir, &Local::today())
                    .unwrap()
                    .as_string()
            );
        }
        SubCommand::Weekly => {
            let mut day = Local::today();
            for _ in 0..7 {
                println!("{}", Schedule::open(&db_dir, &day).unwrap().as_string());
                day = day + Duration::days(1);
            }
        }
        SubCommand::Month => {
            let mut day = Local::today();
            let next_month = if day.month() < 12 { day.month() + 1 } else { 1 };
            let next_month_day = Local.ymd(day.year(), next_month, day.day());
            while day <= next_month_day {
                println!("{}", Schedule::open(&db_dir, &day).unwrap().as_string());
                day = day + Duration::days(1);
            }
        }
        SubCommand::Add => {
            println!("add");
        }
        SubCommand::Remove => {
            println!("remove");
        }
        SubCommand::Update => {
            println!("update");
        }
    }
}
