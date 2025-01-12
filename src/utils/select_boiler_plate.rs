use std::{env, path::PathBuf};

use anyhow::Result;

use crate::{constant, installers::installer::PackageInstallerMap};

use super::{copy_file::copy_file, packages::PackagesEnum};

struct FileConfig {
    template_dir: PathBuf,
    dest_path: PathBuf,
    file_name: String,
}

impl FileConfig {
    fn new(
        project_dir: &PathBuf,
        template_path: &str,
        dest_path: &str,
    ) -> Result<Self> {
        Ok(Self {
            template_dir: env::current_dir()?.join(template_path),
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
        copy_file(&self.template_dir.join(&self.file_name), &self.dest_path)?;
        Ok(())
    }
}

pub fn select_layout_file(
    project_dir: &PathBuf,
    packages: &PackageInstallerMap,
) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        constant::LAYOUT_FILE_TEMPLATE_DIR,
        "src/app/layout.tsx",
    )?;
    file_config.determine_file_name(&packages);
    file_config.copy()?;
    Ok(())
}

pub fn select_page_file(
    project_dir: &PathBuf,
    packages: &PackageInstallerMap,
) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        "template/extras/src/pages/index",
        "src/app/page.tsx",
    )?;
    file_config.determine_file_name(packages);
    file_config.copy()?;
    Ok(())
}

pub fn select_app_file(
    project_dir: &PathBuf,
    packages: &PackageInstallerMap,
) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        "template/extras/src/pages/_app",
        "src/pages/_app.tsx",
    )?;
    file_config.determine_file_name(packages);
    file_config.copy()?;
    Ok(())
}

pub fn select_index_file(
    project_dir: &PathBuf,
    packages: &PackageInstallerMap,
) -> Result<()> {
    let mut file_config = FileConfig::new(
        project_dir,
        "template/extras/src/pages/index",
        "src/pages/index.tsx",
    )?;
    file_config.determine_file_name(packages);
    file_config.copy()?;
    Ok(())
}
