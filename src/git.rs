use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;
use console::Style;
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;
use semver::Version;

use crate::utils::logger::Logger;

const YES_NO_OPTIONS: [&str; 2] = ["Yes", "No"];

pub struct Git {
    pub project_root: PathBuf,
}

impl Git {
    pub fn new(project_dir: &PathBuf) -> Result<Self> {
        Ok(Self {
            project_root: project_dir.to_path_buf(),
        })
    }

    pub fn is_git_repo(&self) -> bool {
        self.project_root.join(".git").is_dir()
    }

    pub fn is_inside_git_repo(&self) -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()?
            .status
            .success();
        Ok(output)
    }

    pub fn get_default_branch(&self) -> Result<String> {
        let config_output = Command::new("git")
            .args(["config", "--global", "init.defaultBranch"])
            .current_dir(&self.project_root)
            .output();
        match config_output {
            Ok(output) if output.status.success() => {
                let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !branch.is_empty() {
                    return Ok(branch);
                }
            }
            _ => {}
        }
        Ok("main".to_string())
    }
}

pub fn initialize_git(project_dir: &PathBuf) -> Result<()> {
    let theme = ColorfulTheme {
        values_style: Style::new().cyan().dim(),
        ..ColorfulTheme::default()
    };
    let config = Git::new(&project_dir)?;
    Logger::info("Initializing Git...");
    let is_git_installed = Command::new("git")
        .arg("version")
        .current_dir(&config.project_root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();
    if !is_git_installed {
        Logger::error("Git is not installed. Skipping Git initialization.");
        return Ok(());
    }
    let is_inside = config.is_inside_git_repo()?;
    if is_inside && config.is_git_repo() {
        let over_write_git: bool = Select::with_theme(&theme).with_prompt(format!("{},Git is already initialized in {:?}. Initializing a new git repository would delete the previous history. Would you like to continue anyways?",String::from("Warning:").red().bold(), project_dir)).items(&YES_NO_OPTIONS).default(1).interact()? == 0;
        if !over_write_git {
            Logger::info("Skipping Git initialization.");
            return Ok(());
        }
        // deleting git folder
        fs::remove_dir_all(config.project_root.join(".git"))?;
    } else if is_inside && !config.is_git_repo() {
        let initialize_child_git_repo: bool = Select::with_theme(&theme).with_prompt(format!("{} {:?} is already in a git worktree. Would you still like to initialize a new git repository in this directory?",String::from("Warning:").red().bold(), project_dir)).items(&YES_NO_OPTIONS).default(1).interact()? == 0;
        if !initialize_child_git_repo {
            Logger::info("Skipping Git initialization.");
            return Ok(());
        }
    }

    let branch_name = config.get_default_branch()?;
    let git_version = config.get_default_branch()?;
    let min_version = Version::parse("2.28.0").unwrap().to_string();
    if git_version < min_version {
        Command::new("git")
            .arg("init")
            .current_dir(&config.project_root)
            .status()?;

        Command::new("git")
            .args([
                "symbolic-ref",
                "HEAD",
                &format!("refs/heads/{}", branch_name),
            ])
            .current_dir(&config.project_root)
            .status()?;
    } else {
        Command::new("git")
            .args(["init", &format!("--initial-branch={}", branch_name)])
            .current_dir(&config.project_root)
            .status()?;
    }

    Command::new("git")
        .args(["add", "."])
        .current_dir(&config.project_root)
        .status()?;
    // fs::remove_dir_all(config.project_root.join(".git"))?;

    Ok(())
}
