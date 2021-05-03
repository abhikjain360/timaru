use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error: unable to open the directory {0}")]
    Dir(PathBuf),
    #[error("error: unable to open file {0}")]
    File(PathBuf),
    #[error("error: environment variables $HOME and $XDG_CONFIG_HOME not set")]
    EnvVar,
    #[error("error: parsing error : {0}")]
    Parse(&'static str),
    #[error("error: invalid index")]
    Idx,
    #[error("error: IO error : {0:?}")]
    IO(#[from] io::Error),
    #[error("error: TUI error : {0:?}")]
    TUI(#[from] crossterm::ErrorKind),
    #[error("error: Log error: {0:?}")]
    Log(#[from] log::SetLoggerError),
}

#[macro_export]
macro_rules! change_parse_err {
    ($res:expr, $text:literal) => {
        match $res {
            Ok(t) => t,
            Err(_) => return Err(Error::Parse($text)),
        }
    };
}
