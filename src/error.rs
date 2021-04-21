use std::path::PathBuf;

#[derive(Debug)]
pub enum TimaruError {
    Dir(PathBuf),
    File(PathBuf),
    EnvVar,
    Parse(&'static str),
}
