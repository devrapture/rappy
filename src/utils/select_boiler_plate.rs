use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};
use include_dir::{include_dir, Dir};

use crate::{constant, installers::installer::PackageInstallerMap};

use super::{ copy_file::copy_file, packages::PackagesEnum};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR");

struct FileConfig {
    template_dir: &'static Dir<'static>,
    dest_path: PathBuf,
    file_name: String,
}

impl FileConfig {
    fn new(project_dir: &PathBuf, template_path: &str, dest_path: &str) -> Result<Self> {
        let template_dir = PROJECT_DIR
            .get_dir(template_path)
            .ok_or_else(|| anyhow!("Template path not found in binary"))?;
        Ok(Self {
            template_dir,
            dest_path: project_dir.join("packages/frontend").join(dest_path),
            file_name: String::from("base.tsx"),
        })
    }

    fn determine_file_name(&mut self, packages: &PackageInstallerMap) {
        let using_tailwind = packages.get(&PackagesEnum::Tailwind).unwrap().in_use;

        self.file_name = match using_tailwind {
            true => "with-tw.tsx",
            false => "base.tsx",
        }
        .to_owned();
    }
    fn copy(&self) -> Result<()> {
        // Retrieve the file from the embedded directory.
        let file = self.template_dir
            .get_file(&self.file_name)
            .ok_or_else(|| anyhow!("Source file '{}' does not exist in embedded directory", self.file_name))?;
        
        // Ensure the destination directory exists.
        if let Some(parent) = self.dest_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }
        
        // Write the embedded file's contents to the destination.
        std::fs::write(&self.dest_path, file.contents())
            .with_context(|| format!("Failed to write file to {:?}", self.dest_path))?;
        
        Ok(())
    }
}

pub fn select_app_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        "template/extras/src/pages/_app",
        "src/pages/_app.tsx",
    )?;
    file_config.determine_file_name(packages);
    file_config
        .copy()?;
    Ok(())
}

pub fn select_layout_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        constant::LAYOUT_FILE_TEMPLATE_DIR,
        "src/app/layout.tsx",
    )?;
    file_config.determine_file_name(&packages);
    file_config.copy()?;
    Ok(())
}

pub fn select_page_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        "template/extras/src/pages/index",
        "src/app/page.tsx",
    )?;
    file_config.determine_file_name(packages);
    file_config.copy()?;
    Ok(())
}

pub fn select_index_file(project_dir: &PathBuf, packages: &PackageInstallerMap) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        "template/extras/src/pages/index",
        "src/pages/index.tsx",
    )?;
    file_config.determine_file_name(packages);
    file_config.copy()?;
    Ok(())
}