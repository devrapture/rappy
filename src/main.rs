mod cli;
mod constant;
mod git;
mod install_packages;
mod scafold_contract;
mod scafold_project;
mod utils;
mod scafold_foundry;

pub mod installers;

use std::process;

use anyhow::{Ok, Result};
use cli::CLiConfig;
use git::initialize_git;
use install_packages::install_frontend_packages;
use installers::{hardhat_prettier::config_hardhat, installer::PackageInstaller};
use scafold_contract::{scafold_foundry, scafold_hardhat};
use utils::{
    logger::Logger,
    rename_project::{rename_foundry_project, rename_frontend_project, rename_hardhat_project, rename_root_project},
};

fn main() {
    if let Err(e) = run() {
        Logger::error(&format!("Error occured {}", e));
        process::exit(1)
    }
}

fn run() -> Result<()> {
    let config = CLiConfig::run()?;
    let use_packages = PackageInstaller::build_pkg_installer_map(&config.packages);
    scafold_project::run(&config.project_dir, &config.project_name)?;
    rename_frontend_project(&config.project_name)?;
    rename_root_project(&config.project_name)?;
    install_frontend_packages(&use_packages, &config.project_dir)?;
    match config.project_type {
        0 => {
            scafold_foundry(&config.project_dir)?;
            rename_foundry_project(&config.project_name)?;
        },
        1 => {
            scafold_hardhat(&config.project_dir)?;
            rename_hardhat_project(&config.project_name)?;
            config_hardhat(&config.project_dir)?;
        },
        _ => unreachable!(),
    }
    if config.initialize_git {
        initialize_git(&config.project_dir)?;
    }
    Ok(())
}
