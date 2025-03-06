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
        .arg(
            Arg::new("flag")
                .long("flag")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("out")
                .long("output")
                //.required(true)
                .action(ArgAction::Set)
                .default_value("-"),
        )
        .arg(Arg::new("cfg").short('c').action(ArgAction::Set))
        .get_matches();

    assert!(cmd.clone().contains_id("flag"));

    let matches = cmd.clone();
    assert!(matches.contains_id("flag"));

    if let Some(c) = matches.get_one::<bool>("flag") {
        println!("Value for --flag: {c}");





        //assert_eq!(matches.get_flag("flag"), true);
    }

    // to get information about the "cfg" argument we created, such as the value supplied we use
    // various ArgMatches methods, such as [ArgMatches::get_one]
    if let Some(c) = matches.get_one::<String>("cfg") {
        println!("Value for -c: {c}");
    }

    // The ArgMatches::get_one method returns an Option because the user may not have supplied
    // that argument at runtime. But if we specified that the argument was "required" as we did
    // with the "out" argument, we can safely unwrap because `clap` verifies that was actually
    // used at runtime.
    println!(
        "Value for --output: {}",
        matches.get_one::<String>("out").unwrap()
    );

    // You can check the presence of an argument's values
    if matches.contains_id("out") {
        // However, if you want to know where the value came from
        if matches.value_source("out").expect("checked contains_id") == ValueSource::CommandLine {
            println!("`out` set by user");
        } else {
            println!("`out` is defaulted");
        }
    }

    color_eyre::install().unwrap();

    let config = CompleteConfig::new()
        .wrap_err("Configuration error.")
        .unwrap();

    if let Some(c) = matches.get_one::<bool>("flag") {
        if matches.get_flag("flag") {
        println!("Value for --flag: {c}");
        terminal::ui_driver(config).await;
        //assert_eq!(matches.get_flag("flag"), true);
    }
    }

    std::process::exit(0)
}
