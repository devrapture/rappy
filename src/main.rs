mod cli;
mod constant;
mod scafold_project;
mod utils;
mod install_packages;

pub mod installers;

use std::process;

use cli::CLiConfig;
use install_packages::install_frontend_packages;
use installers::installer::PackageInstaller;
use utils::{logger::Logger, rename_project::{rename_frontend_project, rename_root_project}};

fn main() {
    let config = match CLiConfig::run() {
        Ok(config) => config,
        Err(e) => {
            Logger::error(&format!("Error {}", e));
            process::exit(1)
        }
    };
    let use_packages = PackageInstaller::build_pkg_installer_map(&config.packages);
    if let Err(e) = scafold_project::run(&config.project_dir, &config.project_name) {
        Logger::error(&format!("Error scafolding {} {}", config.project_name, e));
        process::exit(1)
    }

    if let Err(e) = rename_frontend_project(&config.project_name)  {
        Logger::error(&format!("Error renaming project {} {}", config.project_name, e));
        process::exit(1)
    }

    if let Err(e) = rename_root_project(&config.project_name)  {
        Logger::error(&format!("Error renaming root {} {}", config.project_name, e));
        process::exit(1)
    }


    if let Err(e) =  install_frontend_packages(&use_packages, &config.project_dir) {
        Logger::error(&format!("Error installing frontend packages {}", e));
        process::exit(1)
    }    
}
