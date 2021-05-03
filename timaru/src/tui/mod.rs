use std::{
    io::{self, StdoutLock},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};

use crate::error::Error;

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
}

macro_rules! __impl_change_mode {
    ($($var:path => $f_name:tt => $f_body:expr)+) => {
        impl TimaruTui {
            pub fn change_mode(&mut self, mode: TuiMode) -> Result<(), Error> {
                match mode {
                    $($var => self.$f_name(),)+
                }
            }

            $(fn $f_name(&mut self) -> Result<(), Error> {
                self.mode = $var;
                $f_body(self).map_err(|e| e.into())
            })+

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
    pub fn new() -> Result<Self, Error> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        let stdout = io::stdout();

        // SAFETY: We won't be writing anything to the terminal outside the `TimaruTui` struct.
        Ok(TimaruTui {
            terminal: Terminal::new(CrosstermBackend::new(unsafe {
                std::mem::transmute::<_, StdoutLock<'static>>(stdout.lock())
            }))?,
            mode: TuiMode::Empty,
        })
    }

    // TODO: move this inside `__impl_change_mode` macro and take input about keybindings in the
    // macro.
    pub fn run(mut self) -> Result<(), Error> {
        self.empty_mode()?;

        'outer: loop {
            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    gen_key!(key 'q') => break 'outer,
                    gen_key!(key 'd') => self.day_mode()?,
                    gen_key!(key 'w') => self.week_mode()?,
                    gen_key!(key 'm') => self.month_mode()?,
                    gen_key!(key 'h') => self.empty_mode()?,
                    _ => {}
                }
            } else {
                self.change_mode(self.mode)?;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn testing_stuff(&mut self) -> Result<(), Error> {
        self.terminal
            .draw(|f| {
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
                for day_split in days_splits {
                    f.render_widget(Block::default().borders(Borders::all()), day_split);
                }
            })
            .map_err(|e| e.into())
    }
}

__impl_change_mode! {
    TuiMode::Day => day_mode => |tui: &mut TimaruTui| {
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
            for day_split in days_splits {
                f.render_widget(Block::default().borders(Borders::all()), day_split);
            }
        })
    }

    TuiMode::Month => month_mode => |tui: &mut TimaruTui| {
        tui.terminal.draw(|f| {
            f.render_widget(Block::default().borders(Borders::all()), f.size());
        })
    }
    TuiMode::Week => week_mode => |tui: &mut TimaruTui| {
        tui.terminal.draw(|f| {
            f.render_widget(Block::default().borders(Borders::all()), f.size());
        })
    }
    TuiMode::Empty => empty_mode => |tui: &mut TimaruTui| {
        tui.terminal.draw(|f| {
            let table_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Ratio(1, 10),
                    Constraint::Min(25),
                    Constraint::Ratio(1, 10),
                ])
                .split(f.size());
            info!("cut 1 - {:?}", table_layout[1]);
            let table_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Ratio(1, 15),
                    Constraint::Min(30),
                    Constraint::Ratio(1, 10),
                ])
                .split(table_layout[1]);
            info!("cut 2 - {:?}", table_layout[1]);
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
        })
    }
    TuiMode::Edit => edit_mode => |tui: &mut TimaruTui| {
        tui.terminal.draw(|f| {
            f.render_widget(Block::default().borders(Borders::all()), f.size());
        })
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
