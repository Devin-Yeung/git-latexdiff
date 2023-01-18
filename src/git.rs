use crate::Config;
use git2::build::CheckoutBuilder;
use git2::{Oid, Repository};
use std::path::Path;

pub struct Git<'a> {
    config: &'a Config,
    repo: &'a Repository,
}

impl<'a> Git<'a> {
    pub fn new(config: &'a Config, repo: &'a Repository) -> Self {
        Git { config, repo }
    }

    pub fn checkout_to<P>(&self, commit_id: Oid, target_dir: P)
        where
            P: AsRef<Path>,
    {
        // TODO: Error Handling
        let commit = self.repo.find_commit(commit_id).unwrap();
        let root = commit.tree().unwrap().into_object();

        self.repo.checkout_tree(&root, Some(
            CheckoutBuilder::new()
                .target_dir(target_dir.as_ref())
                .recreate_missing(true)
                .update_index(false) // <= prevent making index messy
        )).unwrap();
    }
}
