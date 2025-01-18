use std::{env, path::PathBuf};

use anyhow::{Context, Result};
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

pub fn install_frontend_packages(
    packages: &PackageInstallerMap,
    project_dir: &PathBuf,
) -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
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
        let index_module_css =
            PathBuf::from(manifest_dir).join(constant::INDEX_MODULE_CSS_TEMPLATE_DIR);
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
