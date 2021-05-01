use timaru::{cli::Opts, error::TimaruError, setup::check_setup};

use clap::Clap;

fn run() -> Result<(), TimaruError> {
    let (_cfg_dir, db_dir) = check_setup()?;

    let opts = Opts::parse();

    #[allow(clippy::single_match)]
    match opts.subcmd {
        Some(subcmd) => subcmd.handle(&db_dir)?,
        None => {
            // TODO: tui starts here
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}
