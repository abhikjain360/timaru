use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    path::{Path, PathBuf},
};

use chrono::{Date, Datelike, Duration, Local};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

use crate::{error::Error, setup::check_dir, task::Task};

#[derive(Clone)]
pub struct Schedule {
    pub date: Date<Local>,
    pub tasks: HashMap<u8, Task>,
    pub file: PathBuf,
}

impl Debug for Schedule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let mut s = format!(
            "# {}-{}-{}\n",
            self.date.day(),
            self.date.month(),
            self.date.year()
        );

        for (idx, task) in self.tasks.iter() {
            s += &format!("({}) {}\n", idx, task.as_string());
        }

        f.write_str(&s)
    }
}

impl Schedule {
    pub async fn open(db_dir: &Path, date: Date<Local>) -> Result<Schedule, Error> {
        let schedule_path = check_dir(
            check_dir(db_dir.join(&format!("{}", date.year())))
                .await?
                .join(&format!("{}", date.month())),
        )
        .await?
        .join(&format!("{}", date.day()));

        let mut schedule_file = BufReader::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&schedule_path)
                .await?,
        );

        let mut schedule_content = String::new();
        schedule_file.read_to_string(&mut schedule_content).await?;

        let schedule_content = schedule_content.trim();

        if schedule_content.is_empty() {
            Ok(Schedule {
                file: schedule_path,
                tasks: Default::default(),
                date,
            })
        } else {
            Schedule::from_str(schedule_path, &schedule_content)
        }
    }

    pub async fn open_range(
        db_dir: &Path,
        start_date: Date<Local>,
        end_date: Date<Local>,
    ) -> Result<Vec<Schedule>, Error> {
        let lenght = (end_date - start_date).num_days();
        let mut futures = Vec::with_capacity(lenght as usize);
        let mut schedules = Vec::with_capacity(lenght as usize);

        for i in 0..lenght {
            let day = start_date + Duration::days(i);
            futures.push(async move { Schedule::open(db_dir, day).await });
        }

        for schedule in futures {
            schedules.push(schedule.await?);
        }

        Ok(schedules)
    }

    pub async fn sync(&self) -> Result<(), Error> {
        let mut schedule_file = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.file)
                .await?,
        );

        schedule_file.write_all(self.as_string().as_bytes()).await?;
        Ok(())
    }

    pub fn flush(&self) -> Result<(), Error> {
        use std::{
            fs,
            io::{self, Write},
        };

        let mut schedule_file = io::BufWriter::new(
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.file)?,
        );

        schedule_file.write_all(self.as_string().as_bytes())?;
        Ok(())
    }

    pub fn add_task(&mut self, task: Task) {
        let idx = self.tasks.len() as u8 + 1;
        self.tasks.insert(idx, task);
    }

    pub fn remove_task(&mut self, idx: u8) -> Option<Task> {
        self.tasks.remove(&idx)
    }
}

impl Drop for Schedule {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}
