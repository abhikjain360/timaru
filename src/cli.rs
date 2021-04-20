use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Abhik Jain <abhikjain360@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Weekly,
    Month,
    Today,
    Update,
    Add,
    Remove,
}
