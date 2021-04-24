use clap::Clap;

#[derive(Clap, Debug, Clone)]
#[clap(version = "0.1")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug, Clone)]
pub enum SubCommand {
    /// Print next 7 days' schedule
    Weekly,
    /// Print schedule from today to next month same day
    Month,
    /// Add a new task
    Add {
        /// The date at which to add a task
        #[clap(long, short)]
        date: Option<String>,
        /// The time at which to add a task
        #[clap(long, short)]
        time: Option<String>,
        /// Whether to enable pomodoro for this task or not
        #[clap(long, short)]
        pomodoro: Option<u8>,
        /// The task description
        description: String,
    },
    /// Remove a task
    Remove {
        /// The date at which to remove a task
        date: String,
        /// The index of task to be removed
        idx: u8,
    },
    /// Update a task
    Update {
        /// The date at which to update a task
        old_date: String,
        /// The idx of the task to update
        idx: u8,
        /// The subcommand to update
        #[clap(subcommand)]
        subcmd: UpdateSubCmd,
    },
    /// View a particular day's schedule. If no argument is provided shows current day's schedule.
    List { date: Option<String> },
}

#[derive(Clap, Debug, Clone)]
pub enum UpdateSubCmd {
    Date {
        date: String,
    },
    /// The time at which to add a task
    Time {
        time: String,
    },
    /// Whether to enable pomodoro for this task or not
    Pomodoro(PomodoroUpdate),
    /// The task description
    Description {
        desc: String,
    },
    /// Mark task as completed
    Done,
    /// Mark task as incomplete
    NotDone
}

#[derive(Clap, Debug, Clone)]
pub enum PomodoroUpdate {
    New { total: u8 },
    Remove,
    Done { done: u8 },
}
