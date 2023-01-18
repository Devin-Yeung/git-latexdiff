mod args;
mod config;
mod git;
mod item;
mod latex;
mod runner;
mod util;

use clap::Parser;

extern crate skim;

use crate::config::Config;
use crate::runner::Runner;

fn main() {
    let args: args::Args = args::Args::parse();
    if args.debug {
        println!("{:#?}", args);
    }

    let config = Config::from(args);

    if config.debug {
        println!("{:#?}", config);
    }

    let runner = Runner::new(config);
    runner.run();
}
