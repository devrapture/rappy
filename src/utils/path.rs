use std::{env, path::PathBuf};

use anyhow::Result;

pub struct PathConfig;

impl PathConfig {
    pub fn new(name: &String) -> Result<PathBuf> {
        let current_dir = env::current_dir()?;
        let path = match name.as_str() {
            "." => &current_dir,
            _ => &current_dir.join(name),
        };
        Ok(path.to_path_buf())
    }
}
