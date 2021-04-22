use clap::Clap;

#[derive(Clap, Debug, Clone)]
#[clap(version = "0.1", author = "Abhik Jain <abhikjain360@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug, Clone)]
pub enum SubCommand {
    /// Print today's schedule
    Today,
    /// Print next 7 days' schedule
    Weekly,
    /// Print schedule from today to next month same day
    Month,
    /// Add a new task
    Add(Add),
    /// Remove a task
    Remove(Remove),
    /// Update a task
    Update(Update),
}

#[derive(Clap, Debug, Clone)]
pub struct Add {
    /// The date at which to add a task
    #[clap(long, short)]
    pub date: Option<String>,
    /// The time at which to add a task
    #[clap(long, short)]
    pub time: Option<String>,
    /// Whether to enable pomodoro for this task or not
    #[clap(long, short)]
    pub pomodoro: Option<u8>,
    /// The task description
    pub description: String,
}

#[derive(Clap, Debug, Clone)]
pub struct Update {
    /// The date at which to add a task
    #[clap(long, short)]
    pub date: Option<String>,
    /// The time at which to add a task
    #[clap(long, short)]
    pub time: Option<String>,
    /// Whether to enable pomodoro for this task or not
    #[clap(long, short)]
    pub pomodoro: Option<u8>,
    /// The task description
    pub description: String,
}

#[derive(Clap, Debug, Clone)]
pub struct Remove {
    /// The date at which to remove a task
    pub date: String,
    /// The index of task to be removed
    pub idx: u8,
}
