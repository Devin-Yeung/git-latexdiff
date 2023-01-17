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
}

impl<'a> LaTeX<'a> {
    pub fn new(config: &'a Config) -> LaTeX<'a> {
        LaTeX { config }
    }

    pub fn find_helper(dir: &PathBuf, ext: &str) -> Vec<PathBuf>
    {
        let mut res = Vec::<PathBuf>::new();

        let paths = fs::read_dir(dir).unwrap();
        for path in paths {
            let path = path.as_ref().unwrap().path();
            if path.extension().unwrap_or_else(|| OsStr::new("")) == ext {
                println!("{}", format!("Found .{}: {}", ext, path.display()).yellow());
                res.push(path);
            }
        }
        if res.is_empty() {
            println!("{}", format!(".{} Not Found!", ext).red());
        }
        return res;
    }

    pub fn get_aux(dir: &PathBuf) -> Option<PathBuf> {
        let mut auxs = self::LaTeX::find_helper(dir, "aux");
        return auxs.pop();
    }

    pub fn get_bbl(dir: &PathBuf) -> Option<PathBuf> {
        let mut bbls = self::LaTeX::find_helper(dir, "bbl");
        return bbls.pop();
    }

    pub fn pdflatex(&self, file: &PathBuf) -> Option<PathBuf> {
        print!("{}", format!("Running pdfLaTeX for {} ...", file.display()).yellow());
        let mut command = Command::new("pdflatex"); // FIXME: specify pdflatex path
        command
            .arg(file)
            .stdout(Stdio::null()) // TODO: Maybe pipe to log?
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .current_dir(file.parent().unwrap());

        let ecode = command.spawn().unwrap().wait().unwrap();

        if ecode.success() {
            println!("{}", "SUCCESS".green());
        } else {
            println!("{}", "FAIL".red());
        }
        None
    }

    /// .aux file should be pass in as `file`
    /// project root dir should be pass in as `porject_dir`
    pub fn bibtex(&self, file: Option<&PathBuf>, project_dir: &PathBuf) -> Option<PathBuf>
    {
        if file.is_none() { return None; }

        let mut file = file.unwrap().to_path_buf();
        file.set_extension("");

        print!("{}", format!("Running bibtex for {} ...", file.display()).yellow());

        let mut command = Command::new("bibtex"); // FIXME: specify bibtex path
        command
            .arg(file)
            .stdout(Stdio::null()) // TODO: Maybe pipe to log?
            .stderr(Stdio::null()) // TODO: Maybe pipe to log?
            .current_dir(project_dir);

        let ecode = command.spawn().unwrap().wait().unwrap();

        if ecode.success() {
            println!("{}", "SUCCESS".green());
        } else {
            println!("{}", "FAIL".red());
        }

        None
    }

    pub fn expand(&self, file: &PathBuf, out: &PathBuf, bbl: Option<&PathBuf>) {
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
            .arg(&out)
            .arg("--expand-bbl")
            .arg(&bbl.unwrap())
            .current_dir(&file.parent().unwrap()); // The working directory should be set
        // TODO: bibtex support

        let ecode = command.spawn().unwrap().wait().unwrap();

        if ecode.success() {
            println!("{}", "SUCCESS".green());
        } else {
            println!("{}", "FAIL".red());
        }
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
