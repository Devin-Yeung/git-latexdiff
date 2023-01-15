mod args;
mod util;
mod git;
mod item;
mod runner;

use args::Args;
use clap::Parser;

extern crate skim;

use skim::prelude::*;
use std::io::Cursor;
use git2::Repository;


fn main() {
    let args: args::Args = args::Args::parse();
    println!("{:#?}", args);

    runner::Runner::run();

    // get working dir
    println!("{}", std::env::current_dir().unwrap().display());

    // checkout_wrapper(oid, &repo);
}


