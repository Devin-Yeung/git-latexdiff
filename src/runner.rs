use crate::git::Git;
use crate::latex::{ConfigBuilder, LaTeX};
use crate::Config;
use crossterm::style::Stylize;
use git2::{Oid, Repository};
use std::fs;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use crate::error::{Error, ErrorKind};
use crate::selector::SelectorBuilder;
use crate::wrapper::CommitWrapper;

pub struct Runner {
    pub config: Config,
    pub repo: Arc<Repository>,
}

impl Runner {
    pub fn new(config: Config) -> std::result::Result<Self, Error> {
        // Repo checker
        let repo = match Repository::discover(&config.repo_dir) {
            Ok(repo) => Arc::new(repo),
            Err(_) => { return Err(Error::new(ErrorKind::RepoNotFound(config.repo_dir))); }
        };

        Ok(Runner { config, repo })
    }

    fn select(&self) -> std::result::Result<(CommitWrapper, CommitWrapper), Error>
    {
        let selector = {
            #[cfg(not(windows))]
            {
                SelectorBuilder::default().repo(self.repo.clone()).build()
            }
            #[cfg(windows)]
            {
                SelectorBuilder::default().build()
            }
        };

        let old_ver = match &self.config.old {
            None => { CommitWrapper::Commit(selector.select()?) }
            Some(x) => { CommitWrapper::parse(&self.repo, x)? }
        };

        let new_ver = match &self.config.new {
            None => { CommitWrapper::Commit(selector.select()?) }
            Some(x) => { CommitWrapper::parse(&self.repo, x)? }
        };

        Ok((old_ver, new_ver))
    }

    pub fn run(&mut self) -> std::result::Result<(), Error> {
        self.prepare();
        // Select
        let (old_ver, new_ver) = self.select()?;
        // Checking out
        info!("{}", "Stage[1/4] Checking Out From Git Repo".yellow().bold().underlined());
        let git = Git::new(&self.config, self.repo.as_ref());
        let mut old_dir = self.config.tmp_dir.clone();
        let mut new_dir = self.config.tmp_dir.clone();
        old_dir.push("old");
        new_dir.push("new");

        git.checkout_to(old_ver, old_dir.as_path());
        git.checkout_to(new_ver, new_dir.as_path());

        info!("{}", "Stage[2/4] Expanding The TeX File".yellow().bold().underlined());
        let tex = LaTeX::new(
            ConfigBuilder::new()
                .project_dir(old_dir.clone())
                .build()?
        );


        tex.pdflatex(None)? // Run pdflatex to generate aux file
            .bibtex(None)?
            .expand(None, None, None)?;
        let old_main_tex = tex.config.main_tex;

        let tex = LaTeX::new(
            ConfigBuilder::new()
                .project_dir(new_dir.clone())
                .build()?
        );

        tex.pdflatex(None)?// Run pdflatex to generate aux file
            .bibtex(None)?
            .expand(None, None, None)?;
        let new_main_tex = tex.config.main_tex;

        // diff two flatten files
        info!("{}", "Stage[3/4] Differing Two Flattened TeX file".yellow().bold().underlined());
        let mut diff_tex = new_main_tex.clone().parent().unwrap().to_path_buf();
        diff_tex.push("diff.tex");
        LaTeX::diff(&self.config, &old_main_tex, &new_main_tex, &diff_tex);

        // building stage
        info!("{}", "Stage[4/4] Compiling Diff Result TeX file".yellow().bold().underlined());

        let tex = LaTeX::new(
            ConfigBuilder::new()
                .project_dir(new_dir.clone())
                .main_tex(diff_tex.clone())
                .build()?
        );

        tex.pdflatex(None)? // Run pdflatex to generate aux file
            .pdflatex(None)?
            .pdflatex(None)?;

        let mut diff_pdf = tex.config.main_tex;
        diff_pdf.set_extension("pdf");

        fs::copy(diff_pdf, &self.config.output).unwrap(); // TODO: add error type
        info!("Diff result placed in {}", &self.config.output.display());

        self.abort(Ok(()));
    }

    fn prepare(&self) {
        // check the tmp dir existence
        let mut tmp_dir = self.config.tmp_dir.clone();
        tmp_dir.push("old");
        fs::create_dir_all(tmp_dir.as_path()).expect("TODO: panic message");
        tmp_dir.pop();

        tmp_dir.push("new");
        fs::create_dir_all(tmp_dir.as_path()).expect("TODO: panic message");
    }

    pub fn abort(&mut self, err: std::result::Result<(), Error>) -> ! {
        // logging
        match err {
            Ok(_) => {}
            Err(e) => { error!("{}", e); }
        }
        // check dangerous operation
        let root = PathBuf::from("/");
        if self.config.tmp_dir == root {
            exit(1);
        }
        // remove the tmp dir
        if !self.config.no_clean {
            fs::remove_dir_all(&self.config.tmp_dir).unwrap();
        }
        exit(0);
    }
}
