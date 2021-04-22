use std::path::PathBuf;

#[derive(Debug)]
pub enum TimaruError {
    Dir(PathBuf),
    File(PathBuf),
    EnvVar,
    Parse(&'static str),
}

#[macro_export]
macro_rules! change_err {
    ($res:expr, $type:literal) => {
        match $res {
            Ok(t) => t,
            Err(_) => return Err(TimaruError::Parse($type)),
        }
    };
}
