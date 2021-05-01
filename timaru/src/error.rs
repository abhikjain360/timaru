use std::{error::Error, fmt, path::PathBuf};

#[derive(Debug)]
pub enum TimaruError {
    Dir(PathBuf),
    File(PathBuf),
    EnvVar,
    Parse(&'static str),
    Idx,
}

#[macro_export]
macro_rules! change_parse_err {
    ($res:expr, $text:literal) => {
        match $res {
            Ok(t) => t,
            Err(_) => return Err(TimaruError::Parse($text)),
        }
    };
}

impl fmt::Display for TimaruError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimaruError::Dir(path) => write!(f, "error: unable to open the directory {:?}", path)?,
            TimaruError::File(path) => write!(f, "error: unable to open file {:?}", path)?,
            TimaruError::EnvVar => write!(
                f,
                "error: environment variables $HOME and $XDG_CONFIG_HOME not set"
            )?,
            TimaruError::Parse(s) => write!(f, "error: file parsing error : {}", s)?,
            TimaruError::Idx => write!(f, "error: invalid index")?,
        }
        Ok(())
    }
}

impl Error for TimaruError {}
