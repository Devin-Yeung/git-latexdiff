use crossterm::style::Stylize;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;

use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::config;
use crate::error::{Error, ErrorKind};
use clap::ValueEnum;
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use walkdir::WalkDir;

pub struct LaTeX {
    pub config: Config,
}

pub struct Config {
    pub project_dir: PathBuf,
    pub main_tex: PathBuf,
    pub abort_if_error: bool,
}

pub struct ConfigBuilder {
    project_dir: PathBuf,
    main_tex: Option<PathBuf>,
    abort_if_error: bool,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder {
            project_dir: std::env::current_dir().unwrap(),
            main_tex: None,
            abort_if_error: false,
        }
    }
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn project_dir(mut self, dir: PathBuf) -> Self {
        self.project_dir = dir;
        self
    }

    pub fn main_tex(mut self, file: PathBuf) -> Self {
        self.main_tex = Some(file);
        self
    }

    pub fn abort_if_error(mut self, on: bool) -> Self {
        self.abort_if_error = on;
        self
    }

    fn guess_main_tex(&self) -> std::result::Result<PathBuf, Error> {
        if self.main_tex.is_some() {
            return Ok(self.main_tex.clone().unwrap());
        }

        warn!("Main TeX file is not given");
        let mut matches = ConfigBuilder::main_searcher(&self.project_dir);
        return match matches.len() {
            0 => {
                warn!("Searcher can't guess the Main TeX file");
                Err(Error::new(ErrorKind::MainTeXNotFound))
            }
            _ => {
                let guess = matches.pop().unwrap();
                info!("Searcher guess main TeX is {}", &guess.display());
                Ok(guess)
            }
        };
    }

    pub fn build(self) -> std::result::Result<Config, Error> {
        let main_tex = self.guess_main_tex()?;
        Ok(Config {
            project_dir: self.project_dir,
            main_tex,
            abort_if_error: self.abort_if_error,
        })
    }

    fn main_searcher(path: &PathBuf) -> Vec<PathBuf> {
        // See https://github.com/BurntSushi/ripgrep/blob/master/crates/grep/examples/simplegrep.rs
        // See https://docs.rs/grep-searcher/0.1.11/grep_searcher/index.html
        let pattern = r"\\documentclass";
        let matcher = RegexMatcher::new_line_matcher(&pattern).unwrap();
        let mut searcher = SearcherBuilder::new()
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .line_number(true)
            .build();

        let mut matches = Vec::<PathBuf>::new();

        for result in WalkDir::new(path) {
            let dent = match result {
                Ok(dent) => dent,
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };
            // Skip if it is not a file or not a tex file
            if !dent.file_type().is_file()
                || dent.path().extension().unwrap_or(OsStr::new("")) != "tex"
            {
                continue;
            }

            let result = searcher.search_path(
                &matcher,
                dent.path(),
                UTF8(|_lnum, _line| {
                    matches.push(dent.path().to_path_buf());
                    Ok(true)
                }),
            );
            if let Err(err) = result {
                eprintln!("{}: {}", dent.path().display(), err);
            }
        }
        matches
    }
}

impl LaTeX {
    pub fn new(config: Config) -> LaTeX {
        LaTeX { config }
    }

    fn ext_finder(&self, ext: &str) -> Vec<PathBuf> {
        let mut res = Vec::<PathBuf>::new();

        let paths = fs::read_dir(&self.config.project_dir).unwrap();
        for path in paths {
            let path = path.as_ref().unwrap().path();
            if path.extension().unwrap_or_else(|| OsStr::new("")) == ext {
                // TODO: Enable me in verbose mode
                info!("Found .{}: {}", ext, path.display());
                res.push(path);
            }
        }
        if res.is_empty() {
            info!(".{} Not Found!", ext);
        }
        return res;
    }

    /// pass in main tex as `file`
    pub fn pdflatex(&self, file: Option<&PathBuf>) -> std::result::Result<&Self, Error> {
        let main_tex = match file {
            Some(file) => file,
            None => &self.config.main_tex,
        };

        info!("Running pdfLaTeX for {}", main_tex.display());
        let mut command = Command::new("pdflatex"); // FIXME: specify pdflatex path
        command
            .arg("-interaction")
            .arg("nonstopmode")
            .arg("-output-directory") // explicitly specify the output directory
            .arg(&self.config.project_dir.as_os_str())
            .arg(main_tex) // main_tex comes the last, the position of args matters on some LaTeX distributions
            .stdout(Stdio::null()) // TODO: Maybe pipe to log?
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .current_dir(&self.config.project_dir); // Run pdflatex in project dir by default

        debug!("CommandLineArgs: {:?}", command);
        debug!("WorkDir: {}", self.config.project_dir.display());

        let ecode = command.spawn().unwrap().wait().unwrap();

        // TODO: Refactor this later
        match (ecode.success(), self.config.abort_if_error) {
            (true, _) => {
                info!("{}", "Compilation SUCCESS".green().bold().underlined())
            }
            (false, false) => {
                warn!("{}", "Compilation FAIL".yellow().bold().underlined())
            }
            (false, true) => {
                error!("{}", "Compilation FAIL".red().bold().underlined());
                return Err(Error::new(ErrorKind::CompileError(String::from(
                    "pdflatex",
                ))));
            }
        }

        Ok(self)
    }

    pub fn bibtex(&self, file: Option<&PathBuf>) -> std::result::Result<&Self, Error> {
        let aux = match file {
            // if aux is not given, find in the project dir
            None => {
                let aux = self.ext_finder("aux").pop();
                if aux.is_none() {
                    return match self.config.abort_if_error {
                        true => Err(Error::new(ErrorKind::CompileError(String::from("bibtex")))), // TODO: Maybe add a new error kind
                        false => Ok(&self),
                    };
                }
                let mut aux = aux.unwrap();
                aux.set_extension("");
                aux
            }
            Some(file) => {
                let mut aux = file.to_owned();
                aux.set_extension("");
                aux
            }
        };

        info!("Running bibtex for {}", aux.display());

        let mut command = Command::new("bibtex"); // FIXME: specify bibtex path

        command
            .arg(&aux)
            .stdout(Stdio::null()) // TODO: Maybe pipe to log?
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .current_dir(&self.config.project_dir);

        debug!("CommandLineArgs: {:?}", command);
        debug!("WorkDir: {}", self.config.project_dir.display());

        let ecode = command.spawn().unwrap().wait().unwrap();

        // TODO: Refactor this later
        match (ecode.success(), self.config.abort_if_error) {
            (true, _) => {
                info!("{}", "Compilation SUCCESS".green().bold().underlined())
            }
            (false, false) => {
                warn!("{}", "Compilation FAIL".yellow().bold().underlined())
            }
            (false, true) => {
                error!("{}", "Compilation FAIL".red().bold().underlined());
                return Err(Error::new(ErrorKind::CompileError(String::from("bibtex"))));
            }
        }

        Ok(&self)
    }

    pub fn expand(
        &self,
        file: Option<&PathBuf>,
        out: Option<&PathBuf>,
        bbl: Option<&PathBuf>,
    ) -> Result<&Self, Error> {
        let file = match file {
            Some(file) => file,
            None => &self.config.main_tex,
        };

        let out = match out {
            Some(out) => out,
            None => &self.config.main_tex,
        };

        let mut real_out = out.to_owned();
        if file == out {
            // if input is same as output name
            // use /foo/bar/_main.tex instead of /foo/bar/main.tex
            // and rename it back after all things done
            // we do this stuff because latexpand is buggy when input file is sample with output file
            let tmp = out.file_name().unwrap();
            real_out.pop();
            real_out.push(format!("_{}", tmp.to_str().unwrap()));
        }

        let bbl = match bbl {
            // if bbl is not given, find in the project dir
            Some(bbl) => bbl.to_owned(),
            None => {
                let bbl = self.ext_finder("bbl").pop();
                if bbl.is_none() {
                    return match self.config.abort_if_error {
                        true => Err(Error::new(ErrorKind::CompileError(String::from(
                            "latexpand",
                        )))), // TODO: Maybe add a new error kind
                        false => Ok(&self),
                    };
                }
                bbl.unwrap()
            }
        };

        info!("Expanding Source: {}", &file.display());
        info!("Expanding Target: {}", &out.display());

        let mut command = Command::new("latexpand"); // FIXME: specify latexpand path

        command
            .arg(&file)
            .arg("--output")
            .arg(&real_out)
            .arg("--expand-bbl")
            .arg(&bbl)
            .current_dir(&file.parent().unwrap()); // The working directory should be set

        debug!("CommandLineArgs: {:?}", command);
        debug!("WorkDir: {}", self.config.project_dir.display());

        let ecode = command.spawn().unwrap().wait().unwrap();

        // TODO: Refactor this later
        match (ecode.success(), self.config.abort_if_error) {
            (true, _) => {
                info!("{}", "Compilation SUCCESS".green().bold().underlined())
            }
            (false, false) => {
                warn!("{}", "Compilation FAIL".yellow().bold().underlined())
            }
            (false, true) => {
                error!("{}", "Compilation FAIL".red().bold().underlined());
                return Err(Error::new(ErrorKind::CompileError(String::from(
                    "latexpand",
                ))));
            }
        }

        if file == out {
            fs::rename(real_out, out).unwrap();
        }

        Ok(self)
    }

    pub fn diff(config: &config::Config, old: &PathBuf, new: &PathBuf, out: &PathBuf) {
        // FIXME: this function need to be refactored
        info!("Diff Source {}", old.display());
        info!("Diff Source {}", new.display());
        info!("Diff Output {}", out.display());
        // pipe to a standalone file
        let diff_result = File::create(out).unwrap();
        let stdio = Stdio::from(diff_result);

        let mut command = Command::new(&config.latexdiff_path);

        command
            .arg(&old)
            .arg(&new)
            .args(&config.latexdiff_args)
            // .arg("--flatten") // FIXME: Sometimes Strange, So remove this args
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .stdout(stdio);

        debug!("CommandLineArgs: {:?}", command);

        let ecode = command.spawn().unwrap().wait().unwrap();

        // TODO: Refactor this later
        match ecode.success() {
            true => {
                info!("{}", "Diff SUCCESS".green().bold().underlined())
            }
            false => {
                error!("{}", "Diff FAIL".red().bold().underlined())
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Engine {
    Pdflatex,
    Xelatex,
    Lualatex,
}
