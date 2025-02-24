use include_dir::{include_dir, Dir};
use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::constant;

static PROJECT_DIR: Dir = include_dir!("template");

pub struct AppRouterConfig {
    pub template_file: &'static include_dir::File<'static>, // Use the embedded file directly
    pub project_root: PathBuf,
}

impl AppRouterConfig {
    pub fn new(project_dir: &PathBuf) -> Result<Self> {
        // Retrieve the embedded file
        let template_file = PROJECT_DIR
            .get_file(constant::APP_ROUTER_TEMPLATE_DIR)
            .ok_or_else(|| anyhow!("App router template file not found in binary"))
            .with_context(|| "Error app router template file not found")?;

        Ok(Self {
            template_file, // Store the embedded file directly
            project_root: project_dir.join("packages/frontend/next.config.js"),
        })
    }

    pub fn copy_file(&self) -> Result<()> {
        // Ensure the destination directory exists
        if let Some(parent) = self.project_root.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the embedded file's contents to the destination
        fs::write(&self.project_root, self.template_file.contents())
            .with_context(|| format!("Failed to write file to {:?}", self.project_root))?;

        Ok(())
    }
}

pub fn install(project_dir: &PathBuf) -> Result<()> {
    let app_router_config = AppRouterConfig::new(&project_dir)?;
    app_router_config.copy_file()?;
    Ok(())
}
