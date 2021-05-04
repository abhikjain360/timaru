use clap::Clap;

use timaru::{cli::Opts, error::Error, log::set_log, setup::check_setup, tui::TimaruTui};

async fn run() -> Result<(), Error> {
    set_log()?;
    let (_cfg_dir, db_dir) = check_setup().await?;

    let opts = Opts::parse();
    match opts.subcmd {
        Some(subcmd) => subcmd.parse(&db_dir).await?,
        None => {
            TimaruTui::new(db_dir)?.run().await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
