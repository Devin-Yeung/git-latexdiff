use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
// #[clap(infer_subcommands(true))]
pub struct Args {
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
    /// Verbose mode
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub verbose: bool,
    /// Do not clean the intermediate files.
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub no_clean: bool,
    /// Specify the path of latexdiff executable
    #[clap(long, value_parser, required(false))]
    pub latexdiff_path: Option<PathBuf>,
    /// Health Check
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub health_check: bool,
    /// Turn debugging information on
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub debug: bool,
}
