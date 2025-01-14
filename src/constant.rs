pub static TITLE_CASE: &'static str = r#"
 ____      _    ____  ______   __
|  _ \    / \  |  _ \|  _ \ \ / /
| |_) |  / _ \ | |_) | |_) \ V /
|  _ <  / ___ \|  __/|  __/ | |
|_| \_\/_/   \_\_|   |_|    |_|

"#;

pub const APP_NAME_PATTERN: &str = r"^[a-z0-9_-]+$|^\.$";
pub const YES_NO_OPTIONS: [&str; 2] = ["Yes", "No"];
pub const EXTRAS_TEMPLATE_DIR: &str = "template/extras";
pub const APP_ROUTER_TEMPLATE_DIR: &str = "template/extras/config/next-config-appdir.js";
pub const TEMPLATE_DIR: &str = "template/root";
pub const HARDHAT_TEMPLATE_DIR: &str = "template/hardhat";
pub const OVERWRITE_OPTIONS: [&str; 2] = ["abort", "clear"];
pub const CONFIRM_OPTIONS: [&str; 2] = ["Yes", "No"];
pub const LAYOUT_FILE_TEMPLATE_DIR: &str = "template/extras/src/app/layout";
pub const INDEX_MODULE_CSS_TEMPLATE_DIR: &str = "template/extras/src/index.module.css";
pub const TAILWIND_CONFIGS: &[(&str, &str)] = &[
    ("config/tailwind.config.ts", "tailwind.config.ts"),
    ("config/postcss.config.js", "postcss.config.js"),
    ("config/_prettier.config.js", "prettier.config.js"),
    ("src/styles/globals.css", "src/styles/globals.css"),
];
