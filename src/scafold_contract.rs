use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::logger::Logger;

// Embed the entire template directories for Hardhat and Foundry.
// Adjust the paths below to point to the correct locations relative to your Cargo.toml.
static HARDHAT_TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template/hardhat");
static FOUNDRY_TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/template/foundry");

struct ProjectConfig {
    project_dir: PathBuf,
}

impl ProjectConfig {
    /// Creates a new ProjectConfig for contract scaffolding.
    /// The contract files will be placed inside `<project_dir>/packages/contract`.
    fn new(project_dir: &PathBuf) -> Self {
        let contract_folder_path = project_dir.join("packages/contract");
        Self {
            project_dir: contract_folder_path,
        }
    }

    /// Copies all files from an embedded template directory to the destination,
    /// preserving the directory structure.
    fn copy_embedded_dir(template: &Dir, destination: &Path) -> Result<()> {
        // Iterate over all files (recursively) in the embedded directory.
        for file in template.files() {
            // The file's path is relative to the embedded directory root.
            let rel_path = file.path();
            let dest_path = destination.join(rel_path);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {:?}", parent))?;
            }
            fs::write(&dest_path, file.contents())
                .with_context(|| format!("Failed to write file: {:?}", dest_path))?;
        }
        Ok(())
    }

    /// Renames a file in the project directory.
    /// For example, renaming a file like `_gitignore` to `.gitignore`.
    fn rename_file(&self, old: &str, new: &str) -> Result<()> {
        let old_file = self.project_dir.join(old);
        let new_file = self.project_dir.join(new);
        fs::rename(&old_file, &new_file).with_context(|| {
            format!(
                "Failed to rename file from {:?} to {:?}",
                old_file, new_file
            )
        })?;
        Ok(())
    }
}

/// Scaffolds the Hardhat contract by copying the embedded Hardhat template
/// into `<project_dir>/packages/contract` and performing any necessary renaming.
pub fn scafold_hardhat(project_dir: &PathBuf) -> Result<()> {
    let config = ProjectConfig::new(project_dir);
    ProjectConfig::copy_embedded_dir(&HARDHAT_TEMPLATE, &config.project_dir)
        .with_context(|| "Failed to copy Hardhat template directory")?;
    config.rename_file("_gitignore", ".gitignore")?;
    Logger::success("✅ Hardhat has been scaffolded successfully");
    Ok(())
}

/// Scaffolds the Foundry contract by copying the embedded Foundry template
/// into `<project_dir>/packages/contract` and performing any necessary renaming.
pub fn scafold_foundry(project_dir: &PathBuf) -> Result<()> {
    let config = ProjectConfig::new(project_dir);
    ProjectConfig::copy_embedded_dir(&FOUNDRY_TEMPLATE, &config.project_dir)
        .with_context(|| "Failed to copy Foundry template directory")?;
    config.rename_file("_gitignore", ".gitignore")?;
    config.rename_file("_gitmodules", ".gitmodules")?;
    Logger::success("✅ Foundry has been scaffolded successfully");
    Ok(())
}
