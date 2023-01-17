use std::ffi::OsStr;
use std::fs;
use crate::Config;
use crossterm::style::Stylize;
use skim::SkimItem;
use std::fs::File;
use std::io::stderr;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use clap::arg;

pub struct LaTeX<'a> {
    config: &'a Config,
    project_dir: &'a PathBuf,
    main_tex: &'a PathBuf, // FIXME: May have lifetime problems?
}

impl<'a> LaTeX<'a> {
    pub fn new(config: &'a Config, project_dir: &'a PathBuf, main_tex: Option<&'a PathBuf>) -> LaTeX<'a> {
        // TODO: if main_tex if not given
        // use main_searcher to find it
        let main_tex = main_tex.unwrap();
        LaTeX { config, project_dir, main_tex }
    }

    fn main_searcher() -> Option<&'a PathBuf> // FIXME: May have lifetime problems?
    {
        // TODO: see https://github.com/BurntSushi/ripgrep/blob/master/crates/grep/examples/simplegrep.rs
        todo!()
    }

    fn ext_finder(&self, ext: &str) -> Vec<PathBuf>
    {
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
    pub fn pdflatex(&self, file: Option<&PathBuf>) -> &Self
    {
        let main_tex = match file {
            Some(file) => { file }
            None => { self.main_tex }
        };

        print!("{}", format!("Running pdfLaTeX for {} ...", main_tex.display()).yellow());
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

    pub fn bibtex(&self, file: Option<&PathBuf>) -> &Self
    {
        let aux = match file {
            // if aux is not given, find in the project dir
            None => {
                let aux = self.ext_finder("aux").pop();
                if aux.is_none() { return &self; }
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
        print!("{}", format!("Running bibtex for {} ...", aux.display()).yellow());

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

    pub fn expand(&self, file: Option<&PathBuf>, out: Option<&PathBuf>, bbl: Option<&PathBuf>) -> &Self
    {
        let file = match file {
            Some(file) => { file }
            None => { self.main_tex }
        };

        let out = match out {
            Some(out) => { out }
            None => { self.main_tex }
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
            Some(bbl) => {
                bbl.to_owned()
            }
            None => {
                let bbl = self.ext_finder("bbl").pop();
                if bbl.is_none() { return &self; } else { bbl.unwrap() }
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

    pub fn diff(&self, old: &PathBuf, new: &PathBuf, out: &PathBuf) {
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

        let mut command = Command::new(&self.config.latexdiff_path);

        command
            .arg(&old)
            .arg(&new)
            .arg("--flatten")
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
