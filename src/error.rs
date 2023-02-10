use std::error;
use std::fmt;
use std::path::PathBuf;

/// An error that can occur in this app.
#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

/// The kind of an error that can occur.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// String here is to represent the name of binary
    BinaryNotFound(String),
    /// String here is to represent the name of task
    CompileError(String),
    /// If user abort the skim when selecting commit id
    /// this error occurs
    SkimAbort,
    /// PathBuf here is to represent the path that searcher
    /// try to find a git repo.
    RepoNotFound(PathBuf),
    /// If the main TeX to be compiled is not given or
    /// can not be inferred (search `\documentclass`)
    /// this error occurs
    MainTeXNotFound,
    /// Functionality is not support on current device,
    /// For example, interactive mode is not support on windows target
    /// since skim does not support windows
    NotSupportedDevice,
    /// The commit hash given by user if invalid
    InvalidCommitHash,
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __NonExhaustive,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::BinaryNotFound(_) => "Executable binary not given and not found in $PATH",
            ErrorKind::CompileError(_) => "Error occurs in compilation.",
            ErrorKind::SkimAbort => "Abort occurs in skim",
            ErrorKind::RepoNotFound(_) => "Repository not given and not found in $PWD",
            ErrorKind::MainTeXNotFound => "Main TeX not given and can not be inferred",
            ErrorKind::NotSupportedDevice => "Not supported device",
            ErrorKind::InvalidCommitHash => "Invalid commit hash",
            ErrorKind::__NonExhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::BinaryNotFound(ref name) => {
                write!(f, "'{}' is not given and not found in $PATH", name)
            }
            ErrorKind::CompileError(ref task) => {
                write!(f, "errors occurs in the {} compilation", task)
            }
            ErrorKind::SkimAbort => {
                write!(f, "abort occurs in the selecting commits")
            }
            ErrorKind::RepoNotFound(ref path) => {
                write!(f, "Repository not found in {}", path.display())
            }
            ErrorKind::MainTeXNotFound => {
                write!(f, "Main TeX not given and can not be inferred")
            }
            ErrorKind::NotSupportedDevice => {
                write!(f, "Not supported device")
            }
            ErrorKind::InvalidCommitHash => {
                write!(f, "Invalid commit hash")
            }
            ErrorKind::__NonExhaustive => unreachable!(),
        }
    }
}
