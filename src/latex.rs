use crate::Config;
use crossterm::style::Stylize;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;

use std::path::PathBuf;
use std::process::{Command, Stdio};

use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use walkdir::WalkDir;
use clap::ValueEnum;

pub struct LaTeX<'a> {
    config: &'a Config,
    project_dir: &'a PathBuf,
    pub main_tex: PathBuf,
}

impl<'a> LaTeX<'a> {
    pub fn new(
        config: &'a Config,
        project_dir: &'a PathBuf,
        main_tex: Option<&'a PathBuf>,
    ) -> Option<LaTeX<'a>> {
        // TODO: if main_tex if not given
        let main_tex = match main_tex {
            Some(path) => path.to_owned(),
            // use main_searcher to find it
            None => {
                print!("{}", "Main TeX file is not given.".yellow());
                let mut matches = LaTeX::main_searcher(project_dir);
                match matches.len() {
                    0 => {
                        println!("{}", "Searcher can't also guess one".red());
                        return None;
                    }
                    _ => {
                        let guess = matches.pop().unwrap();
                        println!(
                            "{}",
                            format!("Searcher guess main TeX is {}", &guess.display()).yellow()
                        );
                        guess
                    }
                }
            }
        };

        Some(LaTeX {
            config,
            project_dir,
            main_tex,
        })
    }

    pub fn main_searcher(path: &PathBuf) -> Vec<PathBuf> {
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

    fn ext_finder(&self, ext: &str) -> Vec<PathBuf> {
        let mut res = Vec::<PathBuf>::new();

        let paths = fs::read_dir(self.project_dir).unwrap();
        for path in paths {
            let path = path.as_ref().unwrap().path();
            if path.extension().unwrap_or_else(|| OsStr::new("")) == ext {
                // TODO: Enable me in verbose mode
                println!("{}", format!("Found .{}: {}", ext, path.display()).yellow());
                res.push(path);
            }
        }
        if res.is_empty() {
            println!("{}", format!(".{} Not Found!", ext).red());
        }
        return res;
    }

    /// pass in main tex as `file`
    pub fn pdflatex(&self, file: Option<&PathBuf>) -> &Self {
        let main_tex = match file {
            Some(file) => file,
            None => &self.main_tex,
        };

        print!(
            "{}",
            format!("Running pdfLaTeX for {} ...", main_tex.display()).yellow()
        );
        let mut command = Command::new("pdflatex"); // FIXME: specify pdflatex path
        command
            .arg(main_tex)
            .arg("-interaction=nonstopmode")
            .stdout(Stdio::null()) // TODO: Maybe pipe to log?
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .current_dir(self.project_dir); // Run pdflatex in project dir by default

        let ecode = command.spawn().unwrap().wait().unwrap();

        if ecode.success() {
            println!("{}", "SUCCESS".green());
        } else {
            println!("{}", "FAIL".red());
        }

        self
    }

    pub fn bibtex(&self, file: Option<&PathBuf>) -> &Self {
        let aux = match file {
            // if aux is not given, find in the project dir
            None => {
                let aux = self.ext_finder("aux").pop();
                if aux.is_none() {
                    return &self;
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
        print!(
            "{}",
            format!("Running bibtex for {} ...", aux.display()).yellow()
        );

        let mut command = Command::new("bibtex"); // FIXME: specify bibtex path

        command
            .arg(&aux)
            .stdout(Stdio::null()) // TODO: Maybe pipe to log?
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .current_dir(self.project_dir);

        let ecode = command.spawn().unwrap().wait().unwrap();

        if ecode.success() {
            println!("{}", "SUCCESS".green());
        } else {
            println!("{}", "FAIL".red());
        }

        &self
    }

    pub fn expand(
        &self,
        file: Option<&PathBuf>,
        out: Option<&PathBuf>,
        bbl: Option<&PathBuf>,
    ) -> &Self {
        let file = match file {
            Some(file) => file,
            None => &self.main_tex,
        };

        let out = match out {
            Some(out) => out,
            None => &self.main_tex,
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
                    return &self;
                } else {
                    bbl.unwrap()
                }
            }
        };

        print!(
            "{}",
            format!(
                "Expanding {}\nTo =====> {} ...",
                &file.display(),
                &out.display()
            )
            .yellow()
        );

        let mut command = Command::new("latexpand"); // FIXME: specify latexpand path

        command
            .arg(&file)
            .arg("--output")
            .arg(&real_out)
            .arg("--expand-bbl")
            .arg(&bbl)
            .current_dir(&file.parent().unwrap()); // The working directory should be set
                                                   // TODO: bibtex support

        let ecode = command.spawn().unwrap().wait().unwrap();

        if ecode.success() {
            println!("{}", "SUCCESS".green());
        } else {
            println!("{}", "FAIL".red());
        }

        if file == out {
            fs::rename(real_out, out).unwrap();
        }

        self
    }

    pub fn diff(config: &Config, old: &PathBuf, new: &PathBuf, out: &PathBuf) {
        print!(
            "{}",
            format!(
                "Compare {}\nAnd     {}\nTo ===> {}...",
                old.display(),
                new.display(),
                out.display()
            )
            .yellow()
        );
        // pipe to a standalone file
        let diff_result = File::create(out).unwrap();
        let stdio = Stdio::from(diff_result);

        let mut command = Command::new(&config.latexdiff_path);

        command
            .arg(&old)
            .arg(&new)
            // .arg("--flatten") // FIXME: Sometimes Strange, So remove this args
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .stdout(stdio);

        let ecode = command.spawn().unwrap().wait().unwrap();

        if ecode.success() {
            println!("{}", "SUCCESS".green());
        } else {
            println!("{}", "FAIL".red());
        }
    }
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Engine {
    Pdflatex,
    Xelatex,
    Lualatex,
}