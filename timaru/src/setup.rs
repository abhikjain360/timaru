use std::{env, path::PathBuf};

use tokio::fs;

use crate::error::Error;

#[inline]
pub async fn config_dir() -> Result<PathBuf, Error> {
    let cfg_dir = if let Ok(dir) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(dir).join("timaru")
    } else if let Ok(dir) = env::var("HOME") {
        PathBuf::from(dir).join(".config/timaru")
    } else {
        return Err(Error::EnvVar);
    };

    check_dir(cfg_dir).await
}

#[inline]
pub async fn check_dir(dir: PathBuf) -> Result<PathBuf, Error> {
    if !dir.is_dir() {
        match fs::create_dir(dir.clone()).await {
            Ok(_) => Ok(dir),
            Err(_) => Err(Error::Dir(dir)),
        }
    } else {
        Ok(dir)
    }
}

#[inline]
pub async fn check_setup() -> Result<(PathBuf, PathBuf), Error> {
    let cfg_dir = config_dir().await?;
    let db_dir = check_dir(cfg_dir.join("db")).await?;

    Ok((cfg_dir, db_dir))
}
