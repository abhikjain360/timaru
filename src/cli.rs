use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Abhik Jain <abhikjain360@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    /// Print today's schedule
    Today,
    /// Print next 7 days' schedule
    Weekly,
    /// Print schedule from today to next month same day
    Month,
    /// Add a new task
    Add,
    /// Remove a task
    Remove,
    /// Update a task
    Update,
}
