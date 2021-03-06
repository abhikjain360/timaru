use std::path::Path;

use chrono::{Datelike, Duration, Local, TimeZone};
use clap::Clap;

use crate::{
    error::Error,
    parser::get_date,
    schedule::Schedule,
    task::{Task, TaskTime},
};

#[derive(Clap, Debug, Clone)]
#[clap(version = "0.1")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Clap, Debug, Clone)]
pub enum SubCommand {
    /// Print next 7 days' schedule
    Week,
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
    NotDone,
}

#[derive(Clap, Debug, Clone)]
pub enum PomodoroUpdate {
    New { total: u8 },
    Remove,
    Done { done: u8 },
}

impl SubCommand {
    pub async fn parse(self, db_dir: &Path) -> Result<(), Error> {
        match self {
            SubCommand::Week => {
                let today = Local::today();
                Schedule::open_range(db_dir, today, today + Duration::days(7))
                    .await?
                    .iter()
                    .for_each(|schedule| println!("{:?}", schedule));
            }
            SubCommand::Month => {
                let today = Local::today();
                let next_month = if today.month() < 12 {
                    today.month() + 1
                } else {
                    1
                };
                let next_month_day = Local.ymd(today.year(), next_month, today.day());
                Schedule::open_range(db_dir, today, next_month_day)
                    .await?
                    .iter()
                    .for_each(|schedule| println!("{:?}", schedule));
            }
            SubCommand::List { date } => {
                let date = match date {
                    Some(date_string) => get_date(&date_string)?,
                    None => Local::today(),
                };
                println!("{:?}", Schedule::open(&db_dir, date).await?);
            }
            SubCommand::Add {
                date,
                time,
                pomodoro,
                description,
            } => {
                let date = match date {
                    Some(date_string) => get_date(&date_string)?,
                    None => Local::today(),
                };

                let task = Task {
                    time: match time {
                        Some(time) => TaskTime::from_str(&time, &date)?,
                        None => TaskTime::Precise { time: Local::now() },
                    },
                    description,
                    pomodoro: pomodoro.map(|total| (total, 0)),
                    finished: false,
                };
                Schedule::open(&db_dir, date).await?.add_task(task);
            }
            SubCommand::Remove { date, idx } => {
                let date = get_date(&date)?;

                if Schedule::open(&db_dir, date)
                    .await?
                    .remove_task(idx)
                    .is_none()
                {
                    return Err(Error::Idx);
                }
            }
            SubCommand::Update {
                old_date,
                idx,
                subcmd,
            } => {
                let old_date = get_date(&old_date)?;
                let mut old_task_schedule = Schedule::open(&db_dir, old_date).await?;

                match subcmd {
                    UpdateSubCmd::Date { date } => {
                        let date = get_date(&date)?;
                        match old_task_schedule.remove_task(idx) {
                            Some(mut task) => {
                                task.time.change_date(&date);
                                Schedule::open(&db_dir, date).await?.add_task(task);
                            }
                            None => return Err(Error::Idx),
                        }
                    }
                    UpdateSubCmd::Time { time } => match old_task_schedule.tasks.get_mut(&idx) {
                        Some(task) => {
                            task.time = TaskTime::from_str(&time, &old_task_schedule.date)?
                        }
                        None => return Err(Error::Idx),
                    },
                    UpdateSubCmd::Description { desc } => {
                        match old_task_schedule.tasks.get_mut(&idx) {
                            Some(task) => task.description = desc,
                            None => return Err(Error::Idx),
                        }
                    }
                    UpdateSubCmd::Pomodoro(pom_update) => match pom_update {
                        PomodoroUpdate::New { total } => {
                            match old_task_schedule.tasks.get_mut(&idx) {
                                Some(task) => {
                                    task.pomodoro = match &task.pomodoro {
                                        Some((_, done)) => Some((total, *done)),
                                        None => Some((total, 0)),
                                    };
                                }
                                None => return Err(Error::Idx),
                            }
                        }
                        PomodoroUpdate::Done { done } => {
                            match old_task_schedule.tasks.get_mut(&idx) {
                                Some(task) => {
                                    task.pomodoro = Some((task.pomodoro.unwrap().0, done))
                                }
                                None => return Err(Error::Idx),
                            }
                        }
                        PomodoroUpdate::Remove => match old_task_schedule.tasks.get_mut(&idx) {
                            Some(task) => task.pomodoro = None,
                            None => return Err(Error::Idx),
                        },
                    },
                    UpdateSubCmd::Done => match old_task_schedule.tasks.get_mut(&idx) {
                        Some(task) => task.finished = true,
                        None => return Err(Error::Idx),
                    },
                    UpdateSubCmd::NotDone => match old_task_schedule.tasks.get_mut(&idx) {
                        Some(task) => task.finished = false,
                        None => return Err(Error::Idx),
                    },
                }
            }
        }

        Ok(())
    }
}
