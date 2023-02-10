use git2::{Oid, Repository};
use crate::error::{Error, ErrorKind};
use crate::runner;
use crossterm::style::Stylize;

#[cfg(not(windows))]
use skim::prelude::*;

#[cfg(not(windows))]
use crate::item::Item;


#[cfg(windows)]
pub struct SelectorBuilder {
    fzf_opts: (),
}

#[cfg(windows)]
pub struct Selector {
    fzf_opts: (),
}

#[cfg(not(windows))]
pub struct SelectorBuilder {
    repo: Option<Arc<Repository>>,
    skim_opts: SkimOptions<'static>,
}

#[cfg(not(windows))]
pub struct Selector {
    repo: Arc<Repository>,
    skim_opts: SkimOptions<'static>,
}

#[cfg(windows)]
impl SelectorBuilder {
    pub fn build(self) -> Selector {
        Selector {
            fzf_opts: self.fzf_opts
        }
    }
}

#[cfg(not(windows))]
impl SelectorBuilder {
    pub fn skim_opts(mut self, skim_opts: SkimOptions<'static>) -> SelectorBuilder {
        self.skim_opts = skim_opts;
        self
    }

    pub fn repo(mut self, repo: Arc<Repository>) -> SelectorBuilder {
        self.repo = Some(repo);
        self
    }

    pub fn build(self) -> Selector {
        Selector {
            repo: self.repo.unwrap(),
            skim_opts: self.skim_opts,
        }
    }
}

impl Default for SelectorBuilder {
    #[cfg(windows)]
    fn default() -> Self {
        SelectorBuilder {
            fzf_opts: ()
        }
    }

    #[cfg(not(windows))]
    fn default() -> Self {
        SelectorBuilder {
            repo: None,
            skim_opts: SkimOptionsBuilder::default()
                .reverse(true)
                .multi(false)
                .preview(Some("")) // preview should be specified to enable preview window
                // .height(Some("50%")) // FIXME: if height is not 100%. it will be buggy
                // See https://github.com/lotabout/skim/issues/494
                .build()
                .unwrap(),
        }
    }
}

#[cfg(windows)]
impl Selector {
    pub fn select(&self) -> std::result::Result<Oid, Error> {
        // TODO: use fzf instead on windows target
        return Err(Error::new(ErrorKind::NotSupportedDevice));
    }
}

#[cfg(not(windows))]
impl Selector {
    fn start(&self) -> SkimOutput {
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

        Skim::run_with(&self.skim_opts, Some(rx)).unwrap()
    }

    fn parse(out: SkimOutput) -> std::result::Result<Oid, Error>
    {
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

    pub fn select(&self) -> std::result::Result<Oid, Error> {
        let skim_out = self.start();
        return Selector::parse(skim_out);
    }
}