use std::path::PathBuf;

#[derive(Debug)]
pub enum TimaruError {
    Dir(PathBuf),
    EnvVar,
    Parse(PathBuf),
}
