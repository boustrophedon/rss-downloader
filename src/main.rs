#[macro_use]
extern crate log;
extern crate fern;

#[macro_use]
extern crate structopt;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate toml;

use structopt::StructOpt;

mod config;
use config::RTConfig;

mod commands;
use commands::{RTArgs, RTCommand};

mod add; use add::add_feed;
mod alias; use alias::add_alias;
mod update; use update::run_update;
mod delete; use delete::delete_feed;

fn main() {
    let args = RTArgs::from_args();

    let logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.level(),
                message
            ))
        })
        .level(level_from_verbosity(args.verbosity))
        .chain(std::io::stdout())
        .apply().unwrap();

    let config = RTConfig::new(args.config);

    if let Some(cmd) = args.cmd {
        match cmd {
            RTCommand::Add(add) => add_feed(add, &config),
            RTCommand::Alias(alias) => add_alias(alias, &config),
            RTCommand::Update => run_update(&config),
            RTCommand::Delete(delete) => delete_feed(delete, &config),
        }
    }

    if args.update {
        run_update(&config);
    }
}

fn level_from_verbosity(verbosity: u64) -> log::LevelFilter {
    match verbosity {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Trace,
    }
}
