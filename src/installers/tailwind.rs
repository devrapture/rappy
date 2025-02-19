use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use include_dir::{include_dir, Dir};

use crate::{
    constant,
    utils::{add_package_dependency::add_package_dependency, package_json::PackageJson},
};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR");

pub struct TailwindConfig {
    pub template_root: PathBuf,
    pub project_root: PathBuf,
}

impl TailwindConfig {
    pub fn new(project_path: &PathBuf) -> Result<Self> {
        let template_dir = PROJECT_DIR
            .get_dir(constant::EXTRAS_TEMPLATE_DIR)
            .ok_or_else(|| anyhow!("Extras template directory not found"))
            .with_context(|| "Error extras template directory not found")?;
        Ok(Self {
            template_root: PathBuf::from(template_dir.path()),
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
            .map_err(|e| anyhow!("Failed to add dependencies: {}", e))
            .with_context(|| "Error failed to add dependencies")?;
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
        // Get the file from the embedded directory.
        let file = self.template_root
            .join(src_relative_path)
            .to_str()
            .and_then(|path| PROJECT_DIR.get_file(path))
            .ok_or_else(|| anyhow!("Source file '{}' not found in template directory", src_relative_path))?;
    
        // Build the destination path.
        let destination = self.project_root.join(dest_relative_path);
    
        // Create parent directories if needed.
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create parent directory: {:?}", parent))?;
        }
    
        // Write the embedded file's contents to the destination.
        fs::write(&destination, file.contents())
            .with_context(|| format!("Failed to write file to destination: {:?}", destination))?;
    
        Ok(())
    }
}

pub fn install(project_path: &PathBuf) -> Result<()> {
    let config_styling = TailwindConfig::new(project_path)
        .with_context(|| "Failed to create TailwindConfig")?;

    config_styling
        .add_package_dependency()
        .with_context(|| "Failed to add package dependencies")?;

    config_styling
        .add_format_scripts()
        .with_context(|| "Failed to add format scripts")?;

    for (src, dest) in constant::TAILWIND_CONFIGS {
        config_styling
            .copy_file(src, dest)
            .with_context(|| format!("Failed to copy file: {} -> {}", src, dest))?;
    }

    Ok(())
}
