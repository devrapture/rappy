use std::{error::Error, fs, path::PathBuf};

use anyhow::Result;

use crate::installers::dependency_versions::get_dependency_version_map;

use super::package_json::PackageJson;

struct AddPackageDependency<'a> {
    dependencies: Vec<&'a str>,
    dev_mode: bool,
    project_dir: PathBuf,
}

impl<'a> AddPackageDependency<'a> {
    pub fn new(dependencies: &Vec<&'a str>, dev_mode: bool, project_dir: &PathBuf) -> Self {
        Self {
            dependencies: dependencies.to_vec(),
            dev_mode,
            project_dir: project_dir.to_path_buf(),
        }
    }

    pub fn add_dependency(&self) -> Result<()> {
        let package_json_path = self.project_dir.join("package.json");
        let content = fs::read_to_string(&package_json_path)?;

        let mut package_json: PackageJson = serde_json::from_str(&content)?;

        let package_dependencies = get_dependency_version_map();
        for dep in &self.dependencies {
            let version = package_dependencies.get(dep);
            if let Some(ver) = version {
                if self.dev_mode {
                    package_json
                        .dev_dependencies
                        .insert(dep.to_string(), ver.to_string());
                } else {
                    package_json
                        .dependencies
                        .insert(dep.to_string(), ver.to_string());
                }
            }
        }

        fs::write(
            &package_json_path,
            serde_json::to_string_pretty(&package_json)?,
        )?;

        Ok(())
    }
}

pub fn add_package_dependency(
    dependencies: &Vec<&str>,
    dev_mode: bool,
    project_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let config = AddPackageDependency::new(&dependencies, dev_mode, &project_dir);
    config.add_dependency()?;
    Ok(())
}
