use crossterm::style::Stylize;
use git2::{Commit, Oid, Repository};
use skim::prelude::*;
use std::collections::VecDeque;

pub struct Item {
    pub repo: Arc<Repository>,
    pub oid: Oid,
}

// See: https://github.com/rust-lang/git2-rs/issues/194
unsafe impl Sync for Item {}

// See: https://github.com/rust-lang/git2-rs/issues/194
unsafe impl Send for Item {}

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        let commit = self.repo.find_commit(self.oid).unwrap();
        let message = commit.message().unwrap();
        let oid = format!("{}", commit.id());
        return Cow::from(String::from(format!("{} {}", oid, message)));
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        let commit = self.repo.find_commit(self.oid).unwrap();
        let summary = commit.summary().unwrap();
        let oid = format!("{}", commit.id());
        return AnsiString::from(String::from(format!("{} {}", &oid[0..7], summary)));
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let commit = self.repo.find_commit(self.oid).unwrap();
        let oid = format!("{}", commit.id());
        ItemPreview::AnsiText(format!(
            "commit {}\nAuthor: {}\n\n{}",
            oid.yellow(),
            commit.author().to_string(),
            commit.message().unwrap()
        ))
    }

    fn output(&self) -> Cow<str> {
        let commit = self.repo.find_commit(self.oid).unwrap();
        let summary = commit.summary().unwrap();
        let oid = format!("{}", commit.id());
        return Cow::from(String::from(format!(
            "{} {} has been selected.",
            &oid[0..7],
            summary
        )));
    }
}
