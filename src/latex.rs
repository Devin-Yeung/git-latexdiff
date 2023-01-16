use crate::Config;
use crossterm::style::Stylize;
use skim::SkimItem;
use std::fs::File;
use std::io::stderr;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

pub struct LaTeX<'a> {
    config: &'a Config,
}

impl<'a> LaTeX<'a> {
    pub fn new(config: &'a Config) -> LaTeX<'a> {
        LaTeX { config }
    }

    pub fn expand(&self, file: &PathBuf, out: &PathBuf) {
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
