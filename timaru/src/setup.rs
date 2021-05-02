use std::{env, path::PathBuf};

use crate::error::Error;

pub fn config_dir() -> Result<PathBuf, Error> {
    let cfg_dir = if let Ok(dir) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(dir).join("timaru")
    } else if let Ok(dir) = env::var("HOME") {
        PathBuf::from(dir).join(".config/timaru")
    } else {
        return Err(Error::EnvVar);
    };

    check_dir(cfg_dir)
}

pub fn check_dir(dir: PathBuf) -> Result<PathBuf, Error> {
    if !dir.is_dir() {
        match std::fs::create_dir(dir.clone()) {
            Ok(_) => Ok(dir),
            Err(_) => Err(Error::Dir(dir)),
        }
    } else {
        Ok(dir)
    }
}

pub fn check_setup() -> Result<(PathBuf, PathBuf), Error> {
    let cfg_dir = config_dir()?;
    let db_dir = check_dir(cfg_dir.join("db"))?;

    Ok((cfg_dir, db_dir))
}
