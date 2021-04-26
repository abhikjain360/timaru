use xshell::cmd;

mod flags;
use flags::{Xtask, XtaskCmd};

fn ci() -> xshell::Result<()> {
    cmd!("cargo fmt").run()?;
    cmd!("cargo clippy").run()?;
    cmd!("cargo build").run()
}

fn main() {
    let flags = Xtask::from_env().expect("unable to parse flags");

    match flags.subcommand {
        XtaskCmd::Ci(_) => {
            ci().unwrap();
        }
        XtaskCmd::Help(_) => {
            println!("{}", Xtask::HELP);
        }
    }
}
