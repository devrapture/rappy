use std::{env, fs};

use anyhow::Result;

use super::package_json::{HardhatPackageJson, PackageJson, RootPackageJson};

pub fn rename_frontend_project(app_name: &String) -> Result<()> {
    let frontend_dir = env::current_dir()?.join(&app_name).join("packages/frontend");
    let package_json_path = frontend_dir.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;
    let mut package_json: PackageJson = serde_json::from_str(&content.as_str())?;
    package_json.name = format!("@{}/frontend",app_name);
    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;
    Ok(())
}

pub fn rename_root_project(app_name: &String) -> Result<()> {
    let root_dir = env::current_dir()?.join(app_name);
    let package_json_path = root_dir.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;
    let mut package_json: RootPackageJson = serde_json::from_str(&content.as_str())?;
    package_json.name = format!("{}",app_name);
    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;
    Ok(())
}

pub fn rename_hardhat_project(app_name:&String) -> Result<()>{
    let root_dir = env::current_dir()?.join(app_name).join("packages/contract");
    let package_json_path = root_dir.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;
    let mut package_json: HardhatPackageJson = serde_json::from_str(&content.as_str())?;
    package_json.name = format!("@{}/frontend",app_name);
    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;
    Ok(())
}