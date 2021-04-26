use xshell::{cwd, cmd, pushd};

mod flags;
use flags::{XtaskCmd, Xtask};

fn ci() -> xshell::Result<()> {
    {
        let _p = pushd("timaru")?;
        cmd!("cargo fmt").run()?;
        cmd!("cargo clippy").run()?;
        cmd!("cargo build").run()
    }
}

fn main() {
    let flags = Xtask::from_env().expect("unable to parse flags");

    match flags.subcommand {
        XtaskCmd::Ci(_) => {
            // TODO: write CI tests here
            ci().unwrap();
        }
        XtaskCmd::Help(_) => {
            println!("{}", Xtask::HELP);
        }
    }
}
