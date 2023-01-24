mod args;
mod config;
mod git;
mod item;
mod latex;
mod runner;
mod util;
mod error;
mod logger;

use clap::Parser;

extern crate skim;

use crate::config::Config;
use crate::logger::Logger;
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

    let mut runner = Runner::new(config).unwrap_or_else(|err| {
        // tmp dir is not created yet
        println!("{}", err);
        std::process::exit(1);
    });

    runner.run().unwrap_or_else(|err| {
        runner.abort(Err(err));
    });

    runner.abort(Ok(()));
}
