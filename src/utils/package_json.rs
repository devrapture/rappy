use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub package_type: String,
    pub private: bool,
    pub scripts: IndexMap<String, String>,
    pub dependencies: IndexMap<String, String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: IndexMap<String, String>,
}
