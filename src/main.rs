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

use clap::{Arg, ArgAction, Command, Parser};
use color_eyre::eyre::{Result, WrapErr};
use rust_project_template::prelude::chat::chat;
use rust_project_template::prelude::evt_loop::evt_loop;
use rust_project_template::prelude::global_rt::global_rt;
use rust_project_template::prelude::terminal;
use rust_project_template::prelude::CompleteConfig;

use rust_project_template::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "user")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[arg(short = 't', long)]
    tui: bool,
    #[arg(long)]
    chat: bool,
    #[arg(long = "cfg", default_value = "")]
    config: String,
}

/// REF: <https://docs.rs/clap/4.5.31/clap/struct.ArgMatches.html#method.subcommand>
///
/// more docs...
///
/// more docs...
///
/// more docs...
///
/// more docs...
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let global_rt_result = global_rt()
        .spawn(async move {
            println!("global_rt async task!");
            evt_loop(/* add args */).await.unwrap();
            //evt_loop(input_rx, peer_tx, topic).await.unwrap();
            String::from("global_rt async task!")
        })
        .await;
    println!("global_rt_result={:?}", global_rt_result?);

    for args in 0..args.count {
        let global_rt_result = global_rt()
            .spawn(async move {
                let evt_loop_result = evt_loop(/* add args */).await.unwrap();
                println!("evt_loop_result! {:?}", &evt_loop_result);
                //evt_loop(input_rx, peer_tx, topic).await.unwrap();
                println!("global_rt async task! {}", &args.clone());
            })
            .await;

        println!("global_rt_result={:?}!", global_rt_result);
    }

    let cmd = Command::new("MyApp")
        .arg(
            Arg::new("name")
                .long("name")
                .short('n')
                //.required(true)
                .action(ArgAction::Set)
                .default_value("-"),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                //.required(true)
                .action(ArgAction::Set)
                .default_value("0"),
        )
        .arg(
            Arg::new("tui")
                .long("tui")
                .short('t')
                //.required(true)
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("chat")
                .long("chat")
                //.required(true)
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(Arg::new("config").long("cfg").action(ArgAction::Set))
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
            terminal::ui_driver(config.clone()).await;
            assert_eq!(matches.get_flag("tui"), true);
        }
    }
    if let Some(c) = matches.get_one::<bool>("chat") {
        if matches.get_flag("chat") {
    let global_rt_result = global_rt()
        .spawn(async move {
            println!("global_rt async task!");
            evt_loop(/* add args */).await.unwrap();
            //evt_loop(input_rx, peer_tx, topic).await.unwrap();
            String::from("global_rt async task!");
            chat().await.expect("")
        })
        .await;
    println!("global_rt_result={:?}", global_rt_result?);
            println!("Value for --chat: {c}");
            terminal::ui_driver(config).await;
            assert_eq!(matches.get_flag("tui"), true);
        }
    }

    std::process::exit(0)
}
