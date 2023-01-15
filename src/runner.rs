use crossterm::style::Stylize;
use git2::{Oid, Repository};
use skim::prelude::*;
use item::Item;
use crate::item;

pub struct Runner {
    // TODO:
}

impl Runner {
    pub fn run() {
        println!("{}", "Please Choose the old version".green());
        let old_oid = self::Runner::select_oid();
        println!("{}", "Please Choose the new version".green());
        let new_oid = self::Runner::select_oid();
    }

    pub fn select_oid() -> Oid {
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
        let repo = match Repository::discover("./") {
            Ok(repo) => Arc::new(repo),
            Err(_) => panic!("No repos found, try to create one?"),
        };

        // Init Channel
        let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

        // Get Commits Walker, from HEAD by default
        let mut walk = repo.revwalk().unwrap();
        walk.push_head().unwrap();

        for oid in walk {
            let _ = tx.send(Arc::from(
                Item { repo: repo.clone(), oid: oid.unwrap() }
            ));
        }

        drop(tx); // Notify Skim

        let mut selected_item = Skim::run_with(&options, Some(rx))
            .map(|out| out.selected_items)
            .unwrap_or_else(Vec::new);

        // TODO: Error Handling
        if selected_item.len() != 1 {
            panic!("More than one items are selected");
        }

        let item = selected_item.pop().unwrap();
        println!("{}", item.output().green());
        let item = (*item).as_any()
            .downcast_ref::<Item>().unwrap();

        return item.oid;
    }
}