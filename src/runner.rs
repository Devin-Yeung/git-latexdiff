use crossterm::style::Stylize;
use git2::{Error, Oid, Repository};
use skim::prelude::*;
use item::Item;
use crate::{Config, item};
use std::fs;
use std::process::exit;
use std::thread::sleep;
use crate::git::Git;

pub struct Runner {
    // TODO:
    pub config: Config,
    pub repo: Arc<Repository>,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        let repo =
            match Repository::discover(&config.repo_dir) {
                Ok(repo) => { Arc::new(repo) }
                Err(_) => { panic!("No repos found, try to create one?") }
            };

        Runner {
            config,
            repo,
        }
    }

    pub fn run(&self) {
        self.prepare();
        println!("{}", "Please Choose the old version".green());
        let old_oid = self.select_oid();
        println!("{}", "Please Choose the new version".green());
        let new_oid = self.select_oid();
        // FIXME: selection can be aborted

        // Checking out
        let git = Git::new(&self.config, self.repo.as_ref());
        let mut tmp_dir = self.config.tmp_dir.clone();
        tmp_dir.push("old");
        git.checkout_to(old_oid, tmp_dir.as_path());
        tmp_dir.pop();
        tmp_dir.push("new");
        git.checkout_to(new_oid, tmp_dir.as_path());
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

    pub fn select_oid(&self) -> Oid {
        // Get options
        // TODO: options should be built in config stage
        let options = SkimOptionsBuilder::default()
            .reverse(true)
            .height(Some("50%"))
            .multi(false)
            .preview(Some("")) // preview should be specified to enable preview window
            .build()
            .unwrap();

        // Get Repo
        // TODO: Repo should be built in config stage
        // let repo = match Repository::discover("./") {
        //     Ok(repo) => Arc::new(repo),
        //     Err(_) => panic!("No repos found, try to create one?"),
        // };

        // Init Channel
        let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

        // Get Commits Walker, from HEAD by default
        let mut walk = self.repo.revwalk().unwrap();
        walk.push_head().unwrap();

        for oid in walk {
            let _ = tx.send(Arc::from(
                Item { repo: self.repo.clone(), oid: oid.unwrap() }
            ));
        }

        drop(tx); // Notify Skim

        let mut selected_item = Skim::run_with(&options, Some(rx))
            .map(|out| out.selected_items)
            .unwrap_or_else(Vec::new);

        // TODO: Error Handling
        if selected_item.len() > 1 {
            println!("{}", "More than one items are selected".red());
            exit(1);
        } else if selected_item.len() <= 0 {
            println!("{}", "No item is selected".red());
            exit(1);
        }

        let item = selected_item.pop().unwrap();
        println!("{}", item.output().green());
        let item = (*item).as_any()
            .downcast_ref::<Item>().unwrap();

        return item.oid;
    }
}