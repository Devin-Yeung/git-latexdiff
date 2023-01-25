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

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

fn main() {
    // Init the global logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug,
                            simplelog::ConfigBuilder::default()
                                .add_filter_allow_str("git_latexdiff")
                                .set_target_level(LevelFilter::Off)
                                .set_thread_level(LevelFilter::Off)
                                .set_time_level(LevelFilter::Off)
                                .set_level_padding(LevelPadding::Right)
                                .build(),
                            TerminalMode::Mixed,
                            ColorChoice::Auto),
        ]
    ).unwrap();

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
