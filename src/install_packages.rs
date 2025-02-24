use anyhow::{anyhow, Context, Result};
use include_dir::{include_dir, Dir};
use owo_colors::OwoColorize;
use std::fs;
use std::path::PathBuf;

use crate::{
    constant,
    installers::installer::PackageInstallerMap,
    utils::{
        logger::Logger,
        packages::PackagesEnum,
        select_boiler_plate::{
            select_app_file, select_index_file, select_layout_file, select_page_file,
        },
    },
};

static PROJECT_DIR: Dir = include_dir!("template");

pub fn install_frontend_packages(
    packages: &PackageInstallerMap,
    project_dir: &PathBuf,
) -> Result<()> {
    Logger::info("Adding boilerplate...");
    for (k, v) in packages {
        if v.in_use {
            let installer_fn = &v.installer;
            installer_fn(&project_dir)?;
            println!(
                "{} {:?}",
                String::from("✅ Successfully setup boilerplate for").green(),
                k.green().bold()
            );
        }
    }

    // If no tailwind, select use css modules
    if !packages.get(&PackagesEnum::Tailwind).unwrap().in_use {
        // Retrieve the embedded file.
        let index_module_css_file = PROJECT_DIR
            .get_file(constant::INDEX_MODULE_CSS_TEMPLATE_DIR)
            .ok_or_else(|| anyhow!("index module css file not found"))
            .with_context(|| "index module css file not found")?;

        // Define the destination path.
        let index_module_css_dest = project_dir
            .join("packages/frontend/src")
            .join(if packages.get(&PackagesEnum::AppRouter).unwrap().in_use {
                "app"
            } else {
                "pages"
            })
            .join("index.module.css");

        // Ensure the destination directory exists.
        if let Some(parent) = index_module_css_dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("unable to create directory at {:?}", parent))?;
        }

        // Check if the file already exists to avoid overwriting.
        if index_module_css_dest.exists() {
            return Err(anyhow!(
                "file already exists at {:?}",
                index_module_css_dest
            ));
        }

        // Write the file's contents to the destination.
        fs::write(&index_module_css_dest, index_module_css_file.contents())
            .with_context(|| "unable to write index module css")?;
    }

    // Select necessary _app,index / layout,page files
    if packages.get(&PackagesEnum::AppRouter).unwrap().in_use {
        select_layout_file(&project_dir, &packages)?;
        select_page_file(&project_dir, &packages).with_context(|| "select page file failed")?;
    } else {
        select_app_file(&project_dir, &packages)?;
        select_index_file(&project_dir, &packages).with_context(|| "select index file failed")?;
    }

    Ok(())
}
