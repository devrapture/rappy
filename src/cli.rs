use std::fmt::Display;

use crate::{
    constant::{APP_NAME_PATTERN, TITLE_CASE, YES_NO_OPTIONS},
    utils::get_theme::get_theme,
};
use anyhow::{Ok, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use owo_colors::OwoColorize;
use regex::Regex;

#[derive(Debug)]
pub enum ProjectType {
    Foundry,
    Hardhat,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::Foundry => write!(f, "Foundry"),
            ProjectType::Hardhat => write!(f, "Hardhat"),
        }
    }
}

pub struct CLiConfig {
    pub theme: ColorfulTheme,
    pub project_name: String,
    pub styling_with_tailwind: bool,
    pub initialize_git: bool,
    pub app_router: bool,
    pub project_type: usize,
}

impl CLiConfig {
    // Main run method to orchestrate the CLI tool
    pub fn run() -> Result<Self> {
        display_banner();

        let config = Self {
            theme:get_theme(),
            project_name: String::new(),
            styling_with_tailwind: false,
            initialize_git: false,
            app_router: false,
            project_type: 0,
        };

        let project_name = config.get_project_name()?;

        let styling_with_tailwind =
            config.prompt_yes_no("Will you be using Tailwind CSS for styling?")?;
        let app_router = config.prompt_yes_no("Would you like to use Next.js App Router?")?;
        let project_type = config.choose_project_type()?;
        let initialize_git =
            config.prompt_yes_no("Should we initialize a Git repository and stage the changes?")?;

        Ok(Self {
            theme:get_theme(),
            project_name,
            styling_with_tailwind,
            initialize_git,
            app_router,
            project_type,
        })
    }

    fn get_project_name(&self) -> Result<String> {
        Input::with_theme(&self.theme)
            .with_prompt("What's the name of your project? \n")
            .validate_with(|input: &String| -> Result<()> {  // Changed return type
                if is_valid_app_name(input) {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("App name must consist of only lowercase alphanumeric characters, '-', and '_'"))
                }
            })
            .interact_text()
            .map_err(|e| anyhow::anyhow!(e))
    }

    fn prompt_yes_no(&self, prompt: &str) -> Result<bool> {
        Ok(Select::with_theme(&self.theme)
            .with_prompt(prompt)
            .items(&YES_NO_OPTIONS)
            .default(0) // Default to the first option ("Yes")
            .interact()?
            == 0)
    }

    fn choose_project_type(&self) -> Result<usize> {
        Select::with_theme(&self.theme)
            .with_prompt("Choose your project type")
            .items(&[ProjectType::Foundry, ProjectType::Hardhat])
            .default(0)
            .interact()
            .map_err(Into::into)
    }
}

// Function to display the banner with style
pub fn display_banner() {
    println!("{}", TITLE_CASE.red().bold());
}

// Function to validate the project name using a regex pattern
fn is_valid_app_name(app_name: &str) -> bool {
    let re = Regex::new(APP_NAME_PATTERN).unwrap();
    re.is_match(app_name)
}
