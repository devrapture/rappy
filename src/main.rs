mod cli;
mod constant;
mod scafold_project;
mod utils;

pub mod installers;

use std::process;

use cli::CLiConfig;
use installers::installer::PackageInstaller;
use utils::logger::Logger;

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
}
