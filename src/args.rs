use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
// #[clap(infer_subcommands(true))]
pub struct Args {
    #[clap(long, short, value_parser, required(false))]
    /// TODO
    pub latexdiff_path: Option<PathBuf>,
    /// TODO
    #[clap(long, short, value_parser, required(false))]
    pub tmp_dir: Option<PathBuf>,
    /// TODO
    #[clap(long, short, value_parser, required(false))]
    pub repo_dir: Option<PathBuf>,
    /// Target Directory to be zipped, current directory by default
    #[clap(long, short, value_parser, required(false))]
    pub input: Option<PathBuf>,
    /// Output file name, {parent_directory}.zip by default
    #[clap(long, short, value_parser, required(false))]
    pub output: Option<PathBuf>,
    /// Max depth in the tree view
    #[clap(long, short, value_parser)]
    pub depth: Option<usize>,
    /// Health Check
    #[clap(long, action = clap::ArgAction::Count)]
    health_check: u8,
    /// Turn debugging information on
    #[clap(long, action = clap::ArgAction::Count)]
    debug: u8,
    /// Dry Run
    #[clap(long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
    /// Ignore hidden files
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub ignore_hidden: bool,
    /// Read gitignore as part of .zipignore
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub read_gitignore: bool,
    /// Wrap all the files with output file name, parent file name by default
    #[clap(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub parent: bool,
}

