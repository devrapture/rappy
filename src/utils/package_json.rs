use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub private: bool,
    #[serde(rename = "type")]
    pub package_type: String,
    pub scripts: IndexMap<String, String>,
    pub dependencies: IndexMap<String, String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: IndexMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct RootPackageJson {
    pub name: String,
    pub private: bool,
    pub scripts: IndexMap<String, String>,
    pub workspaces: Vec<String>,
    pub dependencies: IndexMap<String, String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: IndexMap<String, String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct HardhatPackageJson {
    pub name: String,
    pub scripts: IndexMap<String, String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: IndexMap<String, String>,
    pub dependencies: IndexMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FoundryPackageJson {
    pub name: String,
    pub version: String,
    pub description: String,
    pub main: String,
    pub scripts: IndexMap<String, String>,
    pub keywords: Vec<String>,
    pub author: String,
    pub license: String,
}
