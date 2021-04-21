use clap::Clap;

mod cli;
mod database;
mod error;

use cli::{Opts, SubCommand};
use database::fs::check_setup;

fn main() {
    check_setup().unwrap();

    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Weekly => {
            println!("weekly");
        }
        SubCommand::Month => {
            println!("month");
        }
        SubCommand::Today => {
            println!("today");
        }
        SubCommand::Update => {
            println!("update");
        }
        SubCommand::Add => {
            println!("add");
        }
        SubCommand::Remove => {
            println!("remove");
        }
    }
}
