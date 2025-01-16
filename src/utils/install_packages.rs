use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;

use super::logger::Logger;

pub fn install_packages(project_dir: &PathBuf) -> Result<()> {
    let is_pnpm_installed = Command::new("pnpm")
        .arg("-version")
        .current_dir(&project_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();
    if !is_pnpm_installed {
        Logger::error("pnpm is not installed. Skipping installation.");
        return Ok(());
    }
    Logger::info("Installing dependencies...");
    Command::new("pnpm")
        .arg("install")
        .current_dir(&project_dir)
        .status()?;
    Logger::success("✔ Successfully installed dependencies!");
    Ok(())
}
