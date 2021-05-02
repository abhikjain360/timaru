use clap::Clap;

use timaru::{cli::Opts, error::Error, log::set_log, setup::check_setup, tui::TimaruTui};

fn run() -> Result<(), Error> {
    set_log()?;
    let (_cfg_dir, db_dir) = check_setup()?;

    let opts = Opts::parse();
    match opts.subcmd {
        Some(subcmd) => subcmd.parse(&db_dir)?,
        None => {
            TimaruTui::new()?.run()?;
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
