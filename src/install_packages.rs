use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use include_dir::{include_dir, Dir};
use owo_colors::OwoColorize;

use crate::{
    constant,
    installers::installer::PackageInstallerMap,
    utils::{
        copy_file::copy_file,
        logger::Logger,
        packages::PackagesEnum,
        select_boiler_plate::{
            select_app_file, select_index_file, select_layout_file, select_page_file,
        },
    },
};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR");

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
        let index_module_css_dir = PROJECT_DIR
            .get_dir(constant::INDEX_MODULE_CSS_TEMPLATE_DIR)
            .ok_or_else(|| anyhow!("index module css  directory not found"))?;
        let index_module_css = PathBuf::from(index_module_css_dir.path());
        let index_module_css_dest = project_dir
            .join("packages/frontend/src")
            .join(if packages.get(&PackagesEnum::AppRouter).unwrap().in_use {
                "app"
            } else {
                "pages"
            })
            .join("index.module.css");
        copy_file(&index_module_css, &index_module_css_dest)
            .with_context(|| "unable to install Tailwind css")?;
    }

    // Select necessary _app,index / layout,page files
    if packages.get(&PackagesEnum::AppRouter).unwrap().in_use {
        select_layout_file(&project_dir, &packages)?;
        select_page_file(&project_dir, &packages)?;
    } else {
        select_app_file(&project_dir, &packages)?;
        select_index_file(&project_dir, &packages)?;
    }

    Ok(())
}
