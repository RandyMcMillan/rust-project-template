#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::unused_self,
    clippy::future_not_send
)]

mod commands;
mod handlers;
mod terminal;
mod ui;
mod utils;

use clap::parser::ValueSource;
use clap::{Arg, ArgAction, ArgMatches, Command, Parser, Subcommand};
use color_eyre::eyre::{Result, WrapErr};

use handlers::config::CompleteConfig;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    //let args = Args::parse();

    //for _ in 0..args.count {
    //    println!("Hello {}!", args.name);
    //}

    let cmd = Command::new("MyApp")
        .arg(Arg::new("tui")
                .long("tui")
                //.short('t')
                .action(clap::ArgAction::SetTrue))
        .arg(
            Arg::new("out")
                .long("output")
                //.required(true)
                .action(ArgAction::Set)
                .default_value("-"),
        )
        .arg(Arg::new("cfg").short('c').action(ArgAction::Set))
        .get_matches();

    assert!(cmd.clone().contains_id("tui"));

    let matches = cmd.clone();
    assert!(matches.contains_id("tui"));

    color_eyre::install().unwrap();

    let config = CompleteConfig::new()
        .wrap_err("Configuration error.")
        .unwrap();

    if let Some(c) = matches.get_one::<bool>("tui") {
        if matches.get_flag("tui") {
            println!("Value for --tui: {c}");
            terminal::ui_driver(config).await;
            //assert_eq!(matches.get_flag("tui"), true);
        }
    }

    std::process::exit(0)
}
