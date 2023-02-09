mod args;
mod config;
mod git;
mod latex;
mod runner;
mod util;
mod error;
mod logger;
mod selector;
mod wrapper;

use clap::Parser;

#[cfg(not(windows))]
mod item;

use crate::config::Config;
use crate::runner::Runner;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

fn main() {
    let args: args::Args = args::Args::parse();

    // Init the global logger
    CombinedLogger::init(
        vec![
            TermLogger::new(args.log_level.clone().to_level_filter(),
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

    if args.debug {
        println!("{:#?}", args);
        println!("{:?}", args.log_level)
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
