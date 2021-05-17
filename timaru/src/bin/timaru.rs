use clap::Clap;
use tracing_subscriber;

use timaru::{cli::Opts, error::Error, setup::check_setup, tui::TimaruTui};

async fn run() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
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
