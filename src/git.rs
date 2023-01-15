use std::path::Path;
use git2::{Oid, Repository};
use git2::build::CheckoutBuilder;

fn checkout_wrapper(commit_id: Oid, repo: &Repository) {
    // TODO: Error Handling
    let commit = repo.find_commit(commit_id).unwrap();
    let root = commit.tree().unwrap().into_object();

    let mut cob = CheckoutBuilder::new();
    let target_dir = Path::new("./target/tmp");
    println!("{:?}", target_dir);

    // let cob = cob.target_dir(&target_dir); // TODO: target_dir can be config
    let cob_ref = cob.target_dir(target_dir)
        .recreate_missing(true);


    // repo.checkout_tree(&root, Some(&mut cob)).unwrap();
    repo.checkout_head(Some(cob_ref)).unwrap();
}
