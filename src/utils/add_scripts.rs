use std::{fs, path::PathBuf};

use anyhow::Result;

use super::package_json::RootPackageJson;

pub fn add_script_for_frontend(project_root: &PathBuf, project_name: &String) -> Result<()> {
    let package_json_path = project_root.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;
    let mut package_json: RootPackageJson = serde_json::from_str(&content)?;
    package_json.scripts.insert(
        "dev".to_string(),
        format!(r#"lerna run dev --scope "@{}/frontend""#, project_name),
    );
    package_json.scripts.insert(
        "build".to_string(),
        format!(r#"lerna run build --scope "@{}/frontend""#, project_name),
    );

    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;
    Ok(())
}

pub fn add_script_for_foundry(project_root: &PathBuf, project_name: &String) -> Result<()> {
    let package_json_path = project_root.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;
    let mut package_json: RootPackageJson = serde_json::from_str(&content)?;
    package_json.scripts.insert(
        "forge-format".to_string(),
        format!(
            r#"lerna run forge-format --scope "@{}/contract""#,
            project_name
        ),
    );

    package_json.scripts.insert(
        "forge-test".to_string(),
        format!(
            r#"lerna run forge-test --scope "@{}/contract""#,
            project_name
        ),
    );

    package_json.scripts.insert(
        "forge-coverage".to_string(),
        format!(
            r#"lerna run forge-coverage --scope "@{}/contract""#,
            project_name
        ),
    );

    package_json.scripts.insert(
        "forge-build".to_string(),
        format!(
            r#"lerna run forge-build --scope "@{}/contract""#,
            project_name
        ),
    );

    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;
    Ok(())
}

pub fn add_script_for_hardhart(project_root: &PathBuf, project_name: &String) -> Result<()> {
    let package_json_path = project_root.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;
    let mut package_json: RootPackageJson = serde_json::from_str(&content)?;

    package_json.scripts.insert(
        "compile".to_string(),
        format!(r#"lerna run compile --scope "@{}/contract""#, project_name),
    );

    package_json.scripts.insert(
        "test".to_string(),
        format!(r#"lerna run test --scope "@{}/contract""#, project_name),
    );

    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;
    Ok(())
}
