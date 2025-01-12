use std::{ fs, path::PathBuf};

use anyhow::Result;

pub fn copy_file(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(&src, &dest)?;

    Ok(())
}
