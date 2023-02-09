use crate::args;
use args::Args;
use chrono::prelude::*;

use std::fs;
use std::path::PathBuf;

use derivative::Derivative;
use crate::error::{Error, ErrorKind};

#[cfg(not(windows))]
use skim::prelude::*;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Config {
    pub repo_dir: PathBuf,
    pub tmp_dir: PathBuf,
    pub main_tex: Option<PathBuf>,
    // FIXME: main tex in different version may differ, fix this
    pub latexdiff_path: PathBuf,
    pub output: PathBuf,
    pub verbose: bool,
    pub debug: bool,
    pub no_clean: bool,
    pub cmp2index: bool,
    #[derivative(Debug = "ignore")]
    #[cfg(not(windows))]
    pub skim_opts: SkimOptions<'static>,
}

impl From<Args> for Config {
    fn from(value: Args) -> Self {
        ConfigBuilder::default()
            .repo_dir(value.repo_dir)
            .tmp_dir(value.tmp_dir)
            .latexdiff_path(value.latexdiff_path)
            .main_tex(value.main_tex)
            .output(value.output)
            .verbose(value.verbose)
            .debug(value.debug)
            .no_clean(value.no_clean)
            .cmp2index(value.cmp2index)
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
    verbose: bool,
    no_clean: bool,
    cmp2index: bool,
    debug: bool,
    #[cfg(not(windows))]
    skim_opts: Option<SkimOptions<'static>>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            repo_dir: None,
            tmp_dir: None,
            latexdiff_path: None,
            main_tex: None,
            output: None,
            verbose: false,
            debug: false,
            no_clean: false,
            cmp2index: false,
            #[cfg(not(windows))]
            skim_opts: None,
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
                        std::env::current_dir().unwrap()
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

    pub fn verbose(mut self, on: bool) -> Self {
        self.verbose = on;
        self
    }

    pub fn debug(mut self, on: bool) -> Self {
        self.debug = on;
        self
    }

    pub fn no_clean(mut self, on: bool) -> Self {
        self.no_clean = on;
        self
    }

    pub fn cmp2index(mut self, on: bool) -> Self {
        self.cmp2index = on;
        self
    }

    pub fn output(mut self, path: Option<PathBuf>) -> Self {
        let mut path = match path {
            Some(path) => path,
            None => std::env::current_dir().unwrap(),
        };

        // turn to absolute
        if !path.is_absolute() {
            path = fs::canonicalize(path).unwrap()
        }
        // if given is a dir not a file, specify a file
        if path.is_dir() {
            path.push("diff.pdf");
        }

        self.output = Some(path);
        self
    }

    #[cfg(not(windows))]
    fn skim_default_options() -> SkimOptions<'static> {
        SkimOptionsBuilder::default()
            .reverse(true)
            .multi(false)
            .preview(Some("")) // preview should be specified to enable preview window
            // .height(Some("50%")) // FIXME: if height is not 100%. it will be buggy
            // See https://github.com/lotabout/skim/issues/494
            .build()
            .unwrap()
    }

    #[cfg(not(windows))]
    pub fn skim_opts(mut self, opts: Option<SkimOptions<'static>>) -> Self {
        self.skim_opts = match opts {
            Some(_) => opts,
            None => Some(ConfigBuilder::skim_default_options()),
        };
        self
    }

    pub fn build(self) -> Config {
        Config {
            repo_dir: self.repo_dir.unwrap(),
            tmp_dir: self.tmp_dir.unwrap(),
            main_tex: self.main_tex,
            latexdiff_path: self.latexdiff_path.unwrap(),
            output: self.output.unwrap(),
            verbose: self.verbose,
            debug: self.debug,
            no_clean: self.no_clean,
            cmp2index: self.cmp2index,
            #[cfg(not(windows))]
            skim_opts: self.skim_opts.unwrap(),
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        let mut builder = ConfigBuilder::new()
            .repo_dir(None)
            .tmp_dir(None)
            .latexdiff_path(None)
            .main_tex(None)
            .output(None)
            .verbose(false)
            .no_clean(false)
            .debug(false);

        #[cfg(not(windows))]
        {
            builder = builder.skim_opts(None);
        }

        builder
    }
}
