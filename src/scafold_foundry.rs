use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Ok, Result};

use crate::{constant, utils::logger::Logger};

struct ProjectConfig {
    project_dir: PathBuf,
    template_dir: PathBuf,
}

impl ProjectConfig {
    fn new(project_dir: &PathBuf) -> Result<Self> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let template_dir = PathBuf::from(manifest_dir).join(constant::HARDHAT_TEMPLATE_DIR);
        let contract_folder_path = project_dir.join("packages/contract");
        Ok(Self {
            project_dir: contract_folder_path,
            template_dir,
        })
    }

    fn copy_directory(&self, source: &Path, destination: &Path) -> Result<()> {
        if !destination.try_exists().unwrap() {
            fs::create_dir(destination)?
        }
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dist_path = destination.join(entry.file_name());

            if entry_path.is_dir() {
                self.copy_directory(&entry_path, &dist_path)?;
            } else {
                fs::copy(&entry_path, &dist_path)?;
            }
        }
        Ok(())
    }

    fn rename_gitignore_file(&self, old: &str, new: &str) -> Result<()> {
        let old_file = self.project_dir.join(old);
        let new_file = self.project_dir.join(new);
        fs::rename(&old_file, &new_file)?;
        Ok(())
    }
}

pub fn scafold_hardhat(project_dir: &PathBuf) -> Result<()> {
    let config = ProjectConfig::new(&project_dir)?;
    config.copy_directory(&config.template_dir, &config.project_dir)?;
    config.rename_gitignore_file("_gitignore", ".gitignore")?;
    Logger::success("✅ Hardhart has been scaffolded successfully");
    Ok(())
}
