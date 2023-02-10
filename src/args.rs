use crate::latex;
use crate::logger;
use clap::Parser;
use latex::Engine;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
// #[clap(infer_subcommands(true))]
pub struct Args {
    /// Commit hash of newer version.
    #[clap(long, value_parser, required(false), default_value = None)]
    pub new: Option<String>,
    /// Commit hash of older version.
    #[clap(long, value_parser, required(false), default_value = None)]
    pub old: Option<String>,
    /// Specify the engine that use to compile the documentation.
    #[clap(long, value_enum, required(false))]
    pub engine: Option<Engine>,
    /// Specify the directory to place the intermediate files.
    /// If not given, $PWD/build/tmp by default.
    #[clap(long, short, value_parser, required(false))]
    pub tmp_dir: Option<PathBuf>,
    /// Specify the directory to find the git repository.
    /// We will search it's parent until a repo is found.
    /// If not given, $PWD by default.
    #[clap(long, short, value_parser, required(false))]
    pub repo_dir: Option<PathBuf>,
    /// Specify the tex file to be compiled.
    /// If not given, we will try to guess one.
    /// Most of the time, our guessing is correct.
    #[clap(long, value_parser, required(false))]
    pub main_tex: Option<PathBuf>,
    /// Target file name to place the diff result,
    /// $PWD/diff.pdf by default.
    #[clap(long, short, value_parser, required(false))]
    pub output: Option<PathBuf>,
    /// Do not clean the intermediate files.
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub no_clean: bool,
    /// Specify the log level
    #[clap(long, value_enum, required(false), default_value = "info")]
    pub log_level: logger::LogLevel,
    /// Specify the path of latexdiff executable
    #[clap(long, value_parser, required(false))]
    pub latexdiff_path: Option<PathBuf>,
    /// Health Check
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub health_check: bool,
}
