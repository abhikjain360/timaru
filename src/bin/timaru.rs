use timaru::{
    cli::{Opts, SubCommand},
    database::setup::check_setup,
};

use clap::Clap;

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
