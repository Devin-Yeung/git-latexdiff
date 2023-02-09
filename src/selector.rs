use git2::Oid;
use runner::Runner;
use crate::error::{Error, ErrorKind};
use crate::runner;
use crossterm::style::Stylize;

#[cfg(not(windows))]
use skim::prelude::*;

#[cfg(not(windows))]
use crate::item::Item;


#[cfg(not(windows))]
pub fn select_oid(runner: &mut Runner) -> std::result::Result<Oid, Error> {
    // Init Channel
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    // Get Commits Walker, from HEAD by default
    let mut walk = runner.repo.revwalk().unwrap();
    walk.push_head().unwrap();

    for oid in walk {
        let _ = tx.send(Arc::from(Item {
            repo: runner.repo.clone(),
            oid: oid.unwrap(),
        }));
    }

    drop(tx); // Notify Skim

    let out = Skim::run_with(&runner.config.skim_opts, Some(rx)).unwrap();

    // TODO: return a error instead of aborting
    if out.is_abort {
        return Err(Error::new(ErrorKind::SkimAbort));
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

    return Ok(item.oid);
}


#[cfg(windows)]
pub fn select_oid(runner: &mut Runner) -> std::result::Result<Oid, Error> {
    // TODO: use fzf instead on windows target
    return Err(Error::new(ErrorKind::NotSupportedDevice))
}
