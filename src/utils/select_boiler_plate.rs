use anyhow::{anyhow, Context, Result};
use include_dir::{include_dir, Dir};
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::packages::PackagesEnum;
use crate::installers::installer::PackageInstallerMap;

// Declare the embedded project directory.
static PROJECT_DIR: Dir = include_dir!("template");

struct FileConfig {
    pub template_path: String,
    pub dest_path: PathBuf,
    pub file_name: String,
}

impl FileConfig {
    /// Constructs a new FileConfig.
    fn new(project_dir: &PathBuf, template_path: &str, dest_path: &str) -> Result<Self> {
        Ok(Self {
            template_path: template_path.to_string(),
            dest_path: project_dir.join("packages/frontend").join(dest_path),
            file_name: "base.tsx".to_string(),
        })
    }

    /// Adjusts the file name based on whether Tailwind is in use.
    fn determine_file_name(&mut self, packages: &PackageInstallerMap) {
        let using_tailwind = packages.get(&PackagesEnum::Tailwind).unwrap().in_use;
        self.file_name = if using_tailwind {
            "with-tw.tsx".to_owned()
        } else {
            "base.tsx".to_owned()
        };
    }

    /// Copies the embedded file (using the given template path and file name) to the destination.
    pub fn copy_file(template_path: &str, file_name: &str, destination: &Path) -> Result<()> {
        // Construct the full relative path within the embedded template directory.
        let full_path = format!("{}/{}", template_path, file_name);

        // Retrieve the file from the embedded PROJECT_DIR.
        let file = PROJECT_DIR.get_file(&full_path).ok_or_else(|| {
            anyhow!(
                "Source file '{}' not found in template directory",
                full_path
            )
        })?;

        // Ensure the destination directory exists.
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        // Write the file contents to the destination.
        fs::write(destination, file.contents())
            .with_context(|| format!("Failed to write file to destination: {:?}", destination))?;
        Ok(())
    }
}

pub fn select_layout_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config =
        FileConfig::new(project_dir, "extras/src/app/layout", "src/app/layout.tsx")?;
    file_config.determine_file_name(packages);
    FileConfig::copy_file(
        &file_config.template_path,
        &file_config.file_name,
        &file_config.dest_path,
    )
    .with_context(|| format!("Failed to select layout file"))?;
    Ok(())
}

pub fn select_page_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config =
        FileConfig::new(project_dir, "extras/src/pages/index", "src/app/page.tsx")?;
    file_config.determine_file_name(packages);
    FileConfig::copy_file(
        &file_config.template_path,
        &file_config.file_name,
        &file_config.dest_path,
    )
    .with_context(|| format!("Failed to select page file"))?;
    Ok(())
}

pub fn select_app_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config =
        FileConfig::new(project_dir, "extras/src/pages/_app", "src/pages/_app.tsx")?;
    file_config.determine_file_name(packages);
    FileConfig::copy_file(
        &file_config.template_path,
        &file_config.file_name,
        &file_config.dest_path,
    )
    .with_context(|| format!("Failed to select app file"))?;
    Ok(())
}

pub fn select_index_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config =
        FileConfig::new(project_dir, "extras/src/pages/index", "src/pages/index.tsx")?;
    file_config.determine_file_name(packages);
    FileConfig::copy_file(
        &file_config.template_path,
        &file_config.file_name,
        &file_config.dest_path,
    )
    .with_context(|| format!("Failed to select index file"))?;
    Ok(())
}
