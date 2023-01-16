use std::path::Path;
use git2::{Oid, Repository};
use git2::build::CheckoutBuilder;
use crate::Config;

pub struct Git<'a> {
    config: &'a Config,
    repo: &'a Repository,
}

impl<'a> Git<'a> {
    pub fn new(config: &'a Config, repo: &'a Repository) -> Self {
        Git {
            config,
            repo
        }
    }


    pub fn checkout_to<P>(&self, commit_id: Oid, target_dir: P)
    where P: AsRef<Path>
    {
        // TODO: Error Handling
        let commit = self.repo.find_commit(commit_id).unwrap();
        let root = commit.tree().unwrap().into_object();

        let mut cob = CheckoutBuilder::new();

        let cob = cob.target_dir(target_dir.as_ref())
            .recreate_missing(true);

        self.repo.checkout_tree(&root, Some(cob)).unwrap();
        // self.repo.checkout_head(Some(cob_ref)).unwrap();
    }
}

