use git2::{Object, Oid, Repository};
use crate::error::{Error, ErrorKind};
use crate::wrapper::CommitWrapper::Commit;

pub enum CommitWrapper {
    Index,
    Commit(Oid),
}

impl CommitWrapper {
    pub fn parse(repo: &Repository, hash: &String) -> std::result::Result<CommitWrapper, Error>
    {
        return match hash.to_lowercase().as_str() {
            "index" => Ok(CommitWrapper::Index),
            _ => {
                let res = repo.revparse_single(&hash);
                match res {
                    Ok(x) => { Ok(Commit(x.id())) }
                    Err(_) => { Err(Error::new(ErrorKind::InvalidCommitHash)) }
                }
            }
        };
    }
}