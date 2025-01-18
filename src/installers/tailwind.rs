use std::{env, fs, path::PathBuf};

use anyhow::{anyhow, Result};

use crate::{
    constant,
    utils::{add_package_dependency::add_package_dependency, package_json::PackageJson},
};

pub struct TailwindConfig {
    pub template_root: PathBuf,
    pub project_root: PathBuf,
}

impl TailwindConfig {
    pub fn new(project_path: &PathBuf) -> Result<Self> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let template_dir = PathBuf::from(manifest_dir).join(constant::EXTRAS_TEMPLATE_DIR);
        Ok(Self {
            template_root: template_dir,
            project_root: project_path.join("packages/frontend").to_path_buf(),
        })
    }
    pub fn add_package_dependency(&self) -> Result<()> {
        let dependencies = vec![
            "tailwindcss",
            "postcss",
            "prettier",
            "prettier-plugin-tailwindcss",
        ];
        add_package_dependency(&dependencies, true, &self.project_root)
            .map_err(|e| anyhow!("Failed to add dependencies: {}", e))?;
        Ok(())
    }

    pub fn add_format_scripts(&self) -> Result<()> {
        let package_json_path = self.project_root.join("package.json");
        let content = fs::read_to_string(&package_json_path)?;
        let mut package_json: PackageJson = serde_json::from_str(&content)?;
        package_json.scripts.insert(
            "format:write".to_string(),
            r#"prettier --write "**/*.{ts,tsx,js,jsx,mdx}" --cache"#.to_string(),
        );
        package_json.scripts.insert(
            "format:check".to_string(),
            r#"prettier --check "**/*.{ts,tsx,js,jsx,mdx}" --cache"#.to_string(),
        );
        fs::write(
            &package_json_path,
            serde_json::to_string_pretty(&package_json)?,
        )?;
        Ok(())
    }

    pub fn copy_file(&self, src_relative_path: &str, dest_relative_path: &str) -> Result<()> {
        let source = self.template_root.join(src_relative_path);
        if !source.exists() {
            return Err(anyhow!("Source doesn't exist"));
        }
        let destination = self.project_root.join(dest_relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&source, &destination)?;
        Ok(())
    }
}

pub fn install(project_path: &PathBuf) -> Result<()> {
    let config_styling = TailwindConfig::new(project_path)?;
    config_styling.add_package_dependency()?;
    config_styling.add_format_scripts()?;
    for (src, dest) in constant::TAILWIND_CONFIGS {
        config_styling.copy_file(src, dest)?;
    }

    Ok(())
}
