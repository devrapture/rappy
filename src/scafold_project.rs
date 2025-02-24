use anyhow::{anyhow, Context, Result};
use console::Style;
use include_dir::{include_dir, Dir};
use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

use crate::constant;

static PROJECT_DIR: Dir = include_dir!("template");

struct ProjectConfig {
    path: PathBuf,
    name: String,
    theme: ColorfulTheme,
    // Instead of storing a PathBuf, store a reference to the embedded directory.
    template_dir: &'static Dir<'static>,
}

impl ProjectConfig {
    fn new(project_dir: &PathBuf, app_name: &String) -> Result<Self> {
        let name = project_dir.file_name().map_or_else(
            || String::from(app_name),
            |n| n.to_string_lossy().to_string(),
        );
        let theme = ColorfulTheme {
            values_style: Style::new().cyan().dim(),
            ..ColorfulTheme::default()
        };

        // Look up the embedded subdirectory (e.g., constant::TEMPLATE_DIR might be "templates")
        let template_dir = PROJECT_DIR
            .get_dir(constant::TEMPLATE_DIR)
            .ok_or_else(|| anyhow!("Template directory not found in binary"))?;

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
                String::from("Warning:").red().bold(),
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
            .default(1) // Default to "No"
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

    fn copy_embedded_directory(&self, source: &Dir, destination: &Path) -> Result<()> {
        // The prefix to remove is the path of the `source` directory itself.
        let prefix = source.path();

        // Copy all files directly contained in this directory.
        for file in source.files() {
            // Compute the path relative to the embedded source directory.
            let relative_path = file.path().strip_prefix(prefix).unwrap_or(file.path());
            let dest_path = destination.join(relative_path);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&dest_path, file.contents())?;
        }
        // Then, recursively handle subdirectories.
        for subdir in source.dirs() {
            let relative_path = subdir.path().strip_prefix(prefix).unwrap_or(subdir.path());
            let dest_path = destination.join(relative_path);
            fs::create_dir_all(&dest_path)?;
            // Use the destination for the current subdirectory.
            self.copy_embedded_directory(subdir, &dest_path)?;
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
    let config = ProjectConfig::new(project_dir, app_name)?;
    config.handle_existing_directory()?;
    // Use our new method to copy the embedded templates to the destination directory.
    config
        .copy_embedded_directory(config.template_dir, &config.path)
        .with_context(|| "Template directory does not exist in binary")?;
    config
        .rename_gitignore_file("_gitignore", ".gitignore")
        .with_context(|| "_gitignore file does not exist")?;
    println!(
        "{} {}",
        config.name.cyan().bold(),
        "scaffolded successfully!".green()
    );
    Ok(())
}
