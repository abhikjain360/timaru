use std::{
    io::{self, StdoutLock},
    path::PathBuf,
    time,
};

use chrono::{Duration, Local};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};

use crate::{error::Error, schedule::Schedule};

mod format;
pub use format::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TuiMode {
    Day,
    Week,
    Month,
    Edit,
    Empty,
}

pub type TermType = Terminal<CrosstermBackend<io::StdoutLock<'static>>>;

pub struct TimaruTui {
    terminal: TermType,
    mode: TuiMode,
    db_dir: PathBuf,
}

macro_rules! __impl_change_mode {
    ($($key:tt $val:literal = $var:path => $f_name:tt |$self:ident| $f_body:block)+) => {
        impl TimaruTui {
            pub async fn change_mode(&mut self, mode: TuiMode) -> Result<(), Error> {
                match mode {
                    $($var => self.$f_name().await,)+
                    _ => Ok(())
                }
            }

            $(async fn $f_name(&mut self) -> Result<(), Error> {
                self.mode = $var;
                let $self = self;
                $f_body
            })+

            pub async fn run(mut self) -> Result<(), Error> {
                self.empty_mode().await?;

                'outer: loop {
                    if event::poll(time::Duration::from_millis(100))? {
                        match event::read()? {
                            $(gen_key!($key $val) => self.$f_name().await?,)+
                            gen_key!(key 'q') => break 'outer,
                            _ => {}
                        }
                    } else {
                        self.change_mode(self.mode).await?;
                    }
                }

                Ok(())
            }

        }
    }
}

macro_rules! gen_key {
    (key $val:literal) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($val),
            ..
        })
    };
}

impl TimaruTui {
    pub fn new(db_dir: PathBuf) -> Result<Self, Error> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        let stdout = io::stdout();

        // SAFETY: We won't be writing anything to the terminal outside the `TimaruTui` struct.
        Ok(TimaruTui {
            terminal: Terminal::new(CrosstermBackend::new(unsafe {
                std::mem::transmute::<_, StdoutLock<'static>>(stdout.lock())
            }))?,
            mode: TuiMode::Empty,
            db_dir,
        })
    }

    #[allow(dead_code)]
    async fn testing_stuff(&mut self) -> Result<(), Error> {
        let schedules = Schedule::open_range(
            &self.db_dir,
            Local::today(),
            Local::today() + Duration::days(7),
        )
        .await?
        .into_iter();
        self.terminal.draw(|f| {
            let splits = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(f.size());
            let mut days_splits = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(splits[0]);
            days_splits.extend(
                Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([
                        Constraint::Ratio(1, 4),
                        Constraint::Ratio(1, 4),
                        Constraint::Ratio(1, 4),
                        Constraint::Ratio(1, 4),
                    ])
                    .split(splits[1]),
            );
            for (day_split, day_schedule) in days_splits.into_iter().zip(schedules) {
                f.render_widget(
                    day_schedule
                        .as_widget_paragraph()
                        .block(Block::default().borders(Borders::ALL)),
                    day_split,
                );
            }
        })?;
        Ok(())
    }
}

__impl_change_mode! {
    key 'd' = TuiMode::Day => day_mode |tui| {
        let schedules = Schedule::open_range(
            &tui.db_dir,
            Local::today(),
            Local::today() + Duration::days(7),
        )
        .await?
        .into_iter();
        tui.terminal.draw(|f| {
            let splits = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(f.size());
            let mut days_splits = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(splits[0]);
            days_splits.extend(
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Ratio(1, 4),
                        Constraint::Ratio(1, 4),
                        Constraint::Ratio(1, 4),
                        Constraint::Ratio(1, 4),
                    ])
                    .split(splits[1]),
            );
            for (day_split, day_schedule) in days_splits.into_iter().zip(schedules) {
                let para = day_schedule
                    .as_widget_paragraph()
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(
                    para,
                    day_split,
                );
            }
        })?;
        Ok(())
    }
    key 'h' = TuiMode::Empty => empty_mode |tui| {
        tui.terminal.draw(|f| {
            let table_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Ratio(1, 10),
                    Constraint::Min(25),
                    Constraint::Ratio(1, 10),
                ])
                .split(f.size());
            let table_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Ratio(1, 15),
                    Constraint::Min(30),
                    Constraint::Ratio(1, 10),
                ])
                .split(table_layout[1]);
            let table = Table::new(vec![
                Row::new(vec!["h", "Empty Mode"]),
                Row::new(vec!["q", "Quit"]),
                Row::new(vec!["d", "Day Mode"]),
                Row::new(vec!["w", "Week Mode"]),
                Row::new(vec!["m", "Month Mode"]),
            ])
            .header(Row::new(vec!["Key", "Binding"]).bottom_margin(1))
            .widths(&[Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(table, table_layout[1]);
        })?;
        Ok(())
    }
}

// We need to leave alternate screen and disable raw mode, and thus we implement a drop ourselves.
impl Drop for TimaruTui {
    fn drop(&mut self) {
        // These error don't matter anyway (I hope) as application about to exit.
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
