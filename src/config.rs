use std::fs;
use std::path::{PathBuf};
use git2::Repository;
use skim::prelude::*;
use chrono::prelude::*;
use which::Path;
use args::Args;
use crate::args;

#[derive(Debug)]
pub struct Config {
    pub repo_dir: PathBuf,
    pub tmp_dir: PathBuf,
    pub latexdiff_path: PathBuf,
    // skim_opts: SkimOptions<'static>
}

impl Config {
    fn skim_default_options() -> SkimOptions<'static>
    {
        SkimOptionsBuilder::default()
            .reverse(true)
            .height(Some("50%"))
            .multi(false)
            .preview(Some("")) // preview should be specified to enable preview window
            .build()
            .unwrap()
    }
}

impl From<Args> for Config {
    fn from(value: Args) -> Self {
        ConfigBuilder::new()
            .repo_dir(value.repo_dir)
            .tmp_dir(value.tmp_dir)
            .latexdiff_path(value.latexdiff_path)
            .build()
    }
}

impl Default for Config {
    fn default() -> Self {
        ConfigBuilder::default().build()
    }
}


pub struct ConfigBuilder {
    repo_dir: Option<PathBuf>,
    tmp_dir: Option<PathBuf>,
    latexdiff_path: Option<PathBuf>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            repo_dir: None,
            tmp_dir: None,
            latexdiff_path: None,
        }
    }

    pub fn repo_dir(mut self, path: Option<PathBuf>) -> Self
    {
        match path {
            Some(repo_dir) => {
                // turn all the path to absolute dir
                self.repo_dir = match repo_dir.is_absolute() {
                    true => { Some(repo_dir) }
                    false => { Some(fs::canonicalize(repo_dir).unwrap()) }
                };
            }
            None => {
                self.repo_dir = Some(std::env::current_dir().unwrap())
            }
        }
        self
    }

    pub fn tmp_dir(mut self, path: Option<PathBuf>) -> Self {
        let mut tmp_dir = match path {
            Some(dir) => {
                dir
            }
            None => {
                std::env::current_dir().unwrap()
            }
        };
        let now: DateTime<Local> = Local::now();
        tmp_dir.push(format!("build/tmp/git_latexdiff_{}", now.timestamp()));
        self.tmp_dir = Some(tmp_dir);
        self
    }

    pub fn latexdiff_path(mut self, path: Option<PathBuf>) -> Self {
        self.latexdiff_path = match path {
            None => {
                Some(which::which("latexdiff")
                    .expect("latexdiff not found in $PATH"))
            }
            Some(_) => { path }
        };
        self
    }

    pub fn build(self) -> Config {
        Config {
            repo_dir: self.repo_dir.unwrap(),
            tmp_dir: self.tmp_dir.unwrap(),
            latexdiff_path: self.latexdiff_path.unwrap(),
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
            .repo_dir(None)
            .tmp_dir(None)
            .latexdiff_path(None)
    }
}