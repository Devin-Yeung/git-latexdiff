mod args;
mod config;
mod git;
mod item;
mod latex;
mod runner;
mod util;

use args::Args;
use clap::Parser;

extern crate skim;

use crate::config::Config;
use crate::runner::Runner;
use git2::Repository;
use skim::prelude::*;
use std::io::Cursor;

fn main() {
    let args: args::Args = args::Args::parse();
    println!("{:#?}", args);

    let config = Config::from(args);
    // println!("{:#?}", config);

    let runner = Runner::new(config);
    runner.run();
}
