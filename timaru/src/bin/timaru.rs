#![allow(unused_imports)]

use std::{io, thread, time::Duration};

use clap::Clap;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Clear},
    Terminal,
};

use timaru::{cli::Opts, error::TimaruError, setup::check_setup};

fn run() -> Result<(), TimaruError> {
    enable_raw_mode()?;
    let (_cfg_dir, db_dir) = check_setup()?;

    let opts = Opts::parse();

    #[allow(clippy::single_match)]
    match opts.subcmd {
        Some(subcmd) => subcmd.handle(&db_dir)?,
        None => {
            execute!(io::stdout(), EnterAlternateScreen)?;
            let stdout = io::stdout();
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            loop {
                if event::poll(Duration::from_millis(1000))? {
                    match event::read()? {
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            ..
                        }) => {
                            disable_raw_mode()?;
                            execute!(io::stdout(), LeaveAlternateScreen)?;
                            return Ok(());
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('r'),
                            ..
                        }) => {
                            terminal.draw(|f| {
                                f.render_widget(Block::default().borders(Borders::all()), f.size());
                            })?;
                        }
                        _ => {
                            terminal.draw(|f| {
                                let chunks = Layout::default()
                                    .direction(Direction::Vertical)
                                    .margin(1)
                                    .constraints(
                                        [
                                            Constraint::Percentage(10),
                                            Constraint::Percentage(80),
                                            Constraint::Percentage(10),
                                        ]
                                        .as_ref(),
                                    )
                                    .split(f.size());
                                let block = Block::default().title("Block 1").borders(Borders::ALL);
                                f.render_widget(block, chunks[0]);
                                let block = Block::default().title("Block 2").borders(Borders::ALL);
                                f.render_widget(block, chunks[1]);
                                let block = Block::default().title("Block 3").borders(Borders::ALL);
                                f.render_widget(block, chunks[2]);
                            })?;
                        }
                    }
                }
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() {
    run().unwrap();
}
