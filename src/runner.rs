use crate::git::Git;
use crate::latex::LaTeX;
use crate::{item, Config};
use crossterm::style::Stylize;
use git2::{Oid, Repository};
use item::Item;
use skim::prelude::*;
use std::fs;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;
use crate::error::{Error, ErrorKind};

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

    pub fn run(&mut self) -> std::result::Result<(), Error> {
        self.prepare();
        println!("{}", "Please Choose the old version".green());
        let old_oid = self.select_oid();
        println!("{}", "Please Choose the new version".green());
        let new_oid = self.select_oid();
        // FIXME: selection can be aborted

        // Checking out
        info!("{}", "Stage[1/4] Checking Out From Git Repo".yellow().bold().underlined());
        let git = Git::new(&self.config, self.repo.as_ref());
        let mut old_dir = self.config.tmp_dir.clone();
        let mut new_dir = self.config.tmp_dir.clone();
        old_dir.push("old");
        new_dir.push("new");

        git.checkout_to(old_oid, old_dir.as_path());
        git.checkout_to(new_oid, new_dir.as_path());

        info!("{}", "Stage[2/4] Expanding The TeX File".yellow().bold().underlined());
        let tex = LaTeX::new(&self.config, &old_dir, None)?;


        tex.pdflatex(None) // Run pdflatex to generate aux file
            .bibtex(None)
            .expand(None, None, None);
        let old_main_tex = tex.main_tex;

        let tex = LaTeX::new(&self.config, &new_dir, None)?;

        tex.pdflatex(None) // Run pdflatex to generate aux file
            .bibtex(None)
            .expand(None, None, None);
        let new_main_tex = tex.main_tex;

        // diff two flatten files
        info!("{}", "Stage[3/4] Differing Two Flattened TeX file".yellow().bold().underlined());
        let mut diff_tex = new_main_tex.clone().parent().unwrap().to_path_buf();
        diff_tex.push("diff.tex");
        LaTeX::diff(&self.config, &old_main_tex, &new_main_tex, &diff_tex);

        // building stage
        info!("{}", "Stage[4/4] Compiling Diff Result TeX file".yellow().bold().underlined());
        let tex = LaTeX::new(&self.config, &new_dir, Some(&diff_tex))?;

        tex.pdflatex(None) // Run pdflatex to generate aux file
            .pdflatex(None)
            .pdflatex(None);

        let mut diff_pdf = tex.main_tex;
        diff_pdf.set_extension("pdf");

        let mut out_pdf = std::env::current_dir().unwrap();
        out_pdf.push("diff.pdf");
        fs::copy(diff_pdf, out_pdf).unwrap();

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

    pub fn select_oid(&mut self) -> Oid {
        // Init Channel
        let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

        // Get Commits Walker, from HEAD by default
        let mut walk = self.repo.revwalk().unwrap();
        walk.push_head().unwrap();

        for oid in walk {
            let _ = tx.send(Arc::from(Item {
                repo: self.repo.clone(),
                oid: oid.unwrap(),
            }));
        }

        drop(tx); // Notify Skim

        let out = Skim::run_with(&self.config.skim_opts, Some(rx)).unwrap();

        if out.is_abort {
            self.abort(Err(Error::new(ErrorKind::SkimAbort)));
        }

        let mut selected_item = out.selected_items;

        // TODO: Error Handling
        if selected_item.len() > 1 {
            println!("{}", "More than one items are selected".red());
            unreachable!();
        } else if selected_item.len() <= 0 {
            println!("{}", "No item is selected".red());
            unreachable!();
        }

        let item = selected_item.pop().unwrap();
        println!("{}", item.output().green());
        let item = (*item).as_any().downcast_ref::<Item>().unwrap();

        return item.oid;
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
