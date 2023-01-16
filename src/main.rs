mod args;
mod util;
mod git;
mod item;
mod runner;
mod config;

use args::Args;
use clap::Parser;

extern crate skim;

use skim::prelude::*;
use std::io::Cursor;
use git2::Repository;
use crate::config::Config;
use crate::runner::Runner;


fn main() {
    let args: args::Args = args::Args::parse();
    println!("{:#?}", args);

    let config = Config::from(args);
    println!("{:#?}", config);

    let runner = Runner::new(config);
    runner.run();
}


