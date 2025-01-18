mod cli;
mod constant;
mod git;
mod install_packages;
mod scafold_contract;
mod scafold_foundry;
mod scafold_project;
mod utils;

pub mod installers;

use std::process;

use anyhow::{Context, Ok, Result};
use cli::CLiConfig;
use git::initialize_git;
use install_packages::install_frontend_packages;
use installers::{hardhat_prettier::config_hardhat, installer::PackageInstaller};
use owo_colors::OwoColorize;
use scafold_contract::{scafold_foundry, scafold_hardhat};
use utils::{
    add_scripts::{add_script_for_foundry, add_script_for_frontend, add_script_for_hardhart},
    install_packages::install_packages,
    logger::Logger,
    rename_project::{
        rename_foundry_project, rename_frontend_project, rename_hardhat_project,
        rename_root_project,
    },
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
    rename_root_project(&config.project_name).with_context(|| "Error renaming root project")?;
    add_script_for_frontend(&config.project_dir, &config.project_name)
        .with_context(|| "Error adding frontend script")?;
    install_frontend_packages(&use_packages, &config.project_dir)
        .with_context(|| "Error installing frontend packages")?;
    match config.project_type {
        0 => {
            scafold_foundry(&config.project_dir)?;
            rename_foundry_project(&config.project_name)?;
            add_script_for_foundry(&config.project_dir, &config.project_name)?;
        }
        1 => {
            scafold_hardhat(&config.project_dir)?;
            rename_hardhat_project(&config.project_name)?;
            config_hardhat(&config.project_dir)?;
            add_script_for_hardhart(&config.project_dir, &config.project_name)?;
        }
        _ => unreachable!(),
    }
    if config.install_project {
        install_packages(&config.project_dir)?;
    }
    if config.initialize_git {
        initialize_git(&config.project_dir)?;
    }
    println!(
        "\n✨ {} ✨ \n\n See the README.md file to get started \n\n {}⭐️✨\n\n      {}",
        String::from("Project created").cyan(),
        String::from("Give Rappy a star on Github if you're enjoying it! ").cyan(),
        String::from("https://github.com/devrapture/rappy").cyan()
    );
    Ok(())
}
