use anyhow::{Context, Result};
use console::Style;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

use crate::constant;

struct ProjectConfig {
    path: PathBuf,
    name: String,
    theme: ColorfulTheme,
    template_dir: PathBuf,
}

impl ProjectConfig {
    fn new(project_dir: &PathBuf, app_name: &String) -> Result<Self> {
        let current_dir = env::current_dir()?;
        let name = project_dir
            .file_name()
            .map_or(String::from(app_name), |n| n.to_string_lossy().to_string());
        let theme = ColorfulTheme {
            values_style: Style::new().cyan().dim(),
            ..ColorfulTheme::default()
        };

        let template_dir = current_dir.join(constant::TEMPLATE_DIR);
        Ok(Self {
            path: project_dir.to_path_buf(),
            name,
            theme,
            template_dir,
        })
    }

    fn handle_existing_directory(&self) -> Result<()> {
        if self.path.is_dir() {
            if fs::read_dir(&self.path)?.next().is_none() {
                if self.name != "." {
                    println!(
                        "\n{} exists but is empty, continuing...\n",
                        self.name.bright_red().bold()
                    );
                }
            } else {
                match self.prompt_overwrite_action()? {
                    0 => self.abort(),
                    1 => self.handle_clear_directory()?,
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn prompt_overwrite_action(&self) -> Result<usize> {
        Select::with_theme(&self.theme)
            .with_prompt(format!(
                "\n\n {} {} already exists and isn't empty. How would you like to proceed?",
                format!("{}", String::from("Warning:").red().bold()),
                &self.name.cyan().bold()
            ))
            .items(&constant::OVERWRITE_OPTIONS)
            .default(0)
            .interact()
            .map_err(Into::into)
    }

    fn handle_clear_directory(&self) -> Result<()> {
        println!("Clear the directory and continue installation");
        match Select::with_theme(&self.theme)
            .with_prompt("Are you sure you want to clear the directory?")
            .items(&constant::CONFIRM_OPTIONS)
            .default(1) // Default to the first option ("No")
            .interact()?
        {
            // user selected "yes"
            0 => {
                println!("Emptying {} and creating app..", self.name.cyan().bold());
                fs::remove_dir_all(&self.path)?;
            }
            // user selected "no"
            1 => self.abort(),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn abort(&self) {
        println!("{}", "Aborting installation...".red().bold());
        process::exit(0);
    }

    fn copy_directory(&self, source: &Path, destination: &Path) -> Result<()> {
        if !destination.try_exists().unwrap() {
            fs::create_dir(destination)?
        }
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dist_path = destination.join(entry.file_name());

            if entry_path.is_dir() {
                self.copy_directory(&entry_path, &dist_path)?;
            } else {
                fs::copy(&entry_path, &dist_path)?;
            }
        }
        Ok(())
    }
    fn rename_gitignore_file(&self, old: &str, new: &str) -> Result<()> {
        let dir = self.path.join("packages/frontend");
        let old_file = dir.join(old);
        let new_file = dir.join(new);
        fs::rename(&old_file, &new_file)?;
        Ok(())
    }
}

pub fn run(project_dir: &PathBuf, app_name: &String) -> Result<()> {
    let config = ProjectConfig::new(&project_dir, &app_name)?;
    config.handle_existing_directory()?;
    config
        .copy_directory(&config.template_dir, &config.path)
        .with_context(|| "Source directory does not exist")?;
    config
        .rename_gitignore_file("_gitignore", ".gitignore")
        .with_context(|| "_gitignore file does not exist")?;
    println!(
        "{} {}",
        config.name.cyan().bold(),
        String::from("scaffolded successfully!").green()
    );
    Ok(())
}
