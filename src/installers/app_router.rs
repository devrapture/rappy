use std::{env, fs, path::PathBuf};

use anyhow::{anyhow, Result};

use crate::constant;

pub struct AppRouterConfig {
    pub template_root: PathBuf,
    pub project_root: PathBuf,
}

impl AppRouterConfig {
    pub fn new(project_dir: &PathBuf) -> Result<Self> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let template_dir = PathBuf::from(manifest_dir).join(constant::APP_ROUTER_TEMPLATE_DIR);
        Ok(Self {
            template_root:template_dir,
            project_root: project_dir.join("packages/frontend/next.config.js"),
        })
    }

    pub fn copy_file(
        &self,
        src_relative_path: &PathBuf,
        dest_relative_path: &PathBuf,
    ) -> Result<()> {
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

pub fn install(project_dir: &PathBuf) -> Result<()> {
    let app_router_config = AppRouterConfig::new(&project_dir)?;
    app_router_config.copy_file(
        &app_router_config.template_root,
        &app_router_config.project_root,
    )?;
    Ok(())
}
