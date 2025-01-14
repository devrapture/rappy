use std::{fs, path::PathBuf};

use anyhow::{anyhow, Ok, Result};

use crate::utils::{
    add_package_dependency::add_package_dependency_hardhat, package_json::HardhatPackageJson,
};

struct Config {
    project_root: PathBuf,
}

impl Config {
    fn new(project_path: &PathBuf) -> Result<Self> {
        Ok(Self {
            project_root: project_path.join("packages/contract"),
        })
    }

    fn add_package_dependency(&self) -> Result<()> {
        let dependencies = vec!["prettier"];
        add_package_dependency_hardhat(&dependencies, false, &self.project_root)
            .map_err(|e| anyhow!("Failed to add dependencies into Hardhat: {}", e))?;
        Ok(())
    }

    fn add_format_scripts(&self) -> Result<()> {
        let package_json_path = self.project_root.join("package.json");
        let content = fs::read_to_string(&package_json_path)?;
        let mut package_json: HardhatPackageJson = serde_json::from_str(&content)?;
        package_json.scripts.insert(
            "format".to_string(),
            r#"prettier --write ./*.ts ./deploy/**/*.ts ./scripts/**/*.ts ./test/**/*.ts ./ignition/**/*.ts"#.to_string(),
        );
        fs::write(
            &package_json_path,
            serde_json::to_string_pretty(&package_json)?,
        )?;
        Ok(())
    }
}

pub fn config_hardhat(project_path: &PathBuf) -> Result<()> {
    let config = Config::new(&project_path)?;
    config.add_format_scripts()?;
    config.add_package_dependency()?;
    Ok(())
}
