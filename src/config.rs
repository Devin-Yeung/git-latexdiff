use crate::args;
use args::Args;
use chrono::prelude::*;
use std::ffi::OsString;

use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub repo_dir: PathBuf,
    pub tmp_dir: PathBuf,
    pub latexdiff_path: PathBuf,
    pub output: PathBuf,
    // FIXME: main tex in different version may differ, fix this
    pub main_tex: Option<PathBuf>,
    pub new: Option<String>,
    pub old: Option<String>,
    pub no_clean: bool,
}

impl From<Args> for Config {
    fn from(value: Args) -> Self {
        ConfigBuilder::default()
            .repo_dir(value.repo_dir)
            .tmp_dir(value.tmp_dir)
            .latexdiff_path(value.latexdiff_path)
            .main_tex(value.main_tex)
            .output(value.output)
            .no_clean(value.no_clean)
            .new_hash(value.new)
            .old_hash(value.old)
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
    main_tex: Option<PathBuf>,
    output: Option<PathBuf>,
    new: Option<String>,
    old: Option<String>,
    no_clean: bool,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            repo_dir: None,
            tmp_dir: None,
            latexdiff_path: None,
            main_tex: None,
            output: None,
            new: None,
            old: None,
            no_clean: false,
        }
    }

    pub fn repo_dir(mut self, path: Option<PathBuf>) -> Self {
        let mut path = match path {
            Some(repo_dir) => {
                // turn all the path to absolute dir
                match repo_dir.is_absolute() {
                    true => repo_dir,
                    false => fs::canonicalize(repo_dir).unwrap_or(
                        // if the given relative dir does not exist
                        // fallback to current dir.
                        std::env::current_dir().unwrap(),
                    ),
                }
            }
            None => std::env::current_dir().unwrap(),
        };

        if path.is_file() {
            path.pop();
        }
        self.repo_dir = Some(path);

        self
    }

    pub fn tmp_dir(mut self, path: Option<PathBuf>) -> Self {
        let mut tmp_dir = match path {
            Some(dir) => dir,
            None => std::env::current_dir().unwrap(),
        };
        if tmp_dir.is_file() {
            tmp_dir.pop();
        }
        let now: DateTime<Local> = Local::now();
        // For better compatability
        tmp_dir.push("build");
        tmp_dir.push("tmp");
        tmp_dir.push(format!("git_latexdiff_{}", now.timestamp()));
        self.tmp_dir = Some(tmp_dir);
        self
    }

    pub fn main_tex(mut self, path: Option<PathBuf>) -> Self {
        self.main_tex = path;
        self
    }

    pub fn latexdiff_path(mut self, path: Option<PathBuf>) -> Self {
        self.latexdiff_path = match path {
            None => Some(which::which("latexdiff").expect("latexdiff not found in $PATH")),
            Some(_) => path,
        };
        self
    }

    pub fn new_hash(mut self, hash: Option<String>) -> Self {
        self.new = hash;
        self
    }

    pub fn old_hash(mut self, hash: Option<String>) -> Self {
        self.old = hash;
        self
    }

    pub fn no_clean(mut self, on: bool) -> Self {
        self.no_clean = on;
        self
    }

    pub fn output(mut self, path: Option<PathBuf>) -> Self {
        let mut path = match path {
            Some(path) => path,
            None => std::env::current_dir().unwrap(),
        };

        let file_name = match path.is_file() {
            true => OsString::from(path.file_name().unwrap()),
            false => OsString::from("diff.pdf"),
        };

        if path.is_file() {
            path.pop();
        }

        // turn to absolute
        if !path.is_absolute() {
            path = fs::canonicalize(path).unwrap()
        }
        path.push(file_name);

        self.output = Some(path);
        self
    }

    pub fn build(self) -> Config {
        Config {
            repo_dir: self.repo_dir.unwrap(),
            tmp_dir: self.tmp_dir.unwrap(),
            main_tex: self.main_tex,
            latexdiff_path: self.latexdiff_path.unwrap(),
            output: self.output.unwrap(),
            new: self.new,
            old: self.old,
            no_clean: self.no_clean,
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        let builder = ConfigBuilder::new()
            .repo_dir(None)
            .tmp_dir(None)
            .latexdiff_path(None)
            .main_tex(None)
            .output(None)
            .no_clean(false);

        builder
    }
}
