mod app;
mod spinner;
mod ui;
mod config;

use std::{env, process};
use crate::app::App;
use crate::config::Config;
use anyhow::Result;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    println!("Away v{}", VERSION);

    let config = Config::load_config()?;
    let args: Vec<String> = env::args().collect();

    ctrlc::set_handler(move || {
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    parse_arguments(&args, config)?;

    Ok(())
}

fn parse_arguments(args: &Vec<String>, config: Config) -> Result<()> {
    const TIME_ARGUMENT: &str = "-t";
    const DURATION: &str = "-d";
    const INDEFINITELY: &str = "-i";
    const HELP: &str = "-h";
    const VERSION: &str = "-v";

    if args.contains(&TIME_ARGUMENT.to_string()) == true {
        let index = args.iter().position(|r| r == &TIME_ARGUMENT.to_string()).unwrap();
        if args.len() > index + 1 {
            let mut app = App::new(config);
            app.run_till_time(&args)?;
        }
        else {
            eprintln!("Error: missing time parameter");
            println!();
            display_usage();
        }
    }
    else if args.contains(&DURATION.to_string()) == true {
        let index = args.iter().position(|r| r == &DURATION.to_string()).unwrap();
        if args.len() > index + 1 {
            let mut app = App::new(config);
            app.run_duration(&args)?;
        }
        else {
            eprintln!("Error: missing duration parameter");
            println!();
            display_usage();
        }
    }
    else if args.contains(&INDEFINITELY.to_string()) == true {
        let mut app = App::new(config);
        app.run_indefinitely();
    }
    else if args.contains(&HELP.to_string()) == true {
        display_usage();
    }
    else if args.contains(&VERSION.to_string()) == true {
        display_version();
    }
    else {
        eprintln!("Error: Argument not valid");
        println!();
        display_usage();
    }

    Ok(())
}

fn display_usage() {
    let version = env!("CARGO_PKG_VERSION");

    println!("away {}", version);
    println!("Away management tool");
    println!("Author: Carman Babin");
    println!();
    println!("USAGE:");
    println!("    away [OPTION] <ARGUMENT>");
    println!();
    println!("FLAGS:");
    println!("    -h\t\t\tPrints help information");
    println!("    -v\t\t\tPrints version information");
    println!();
    println!("OPTIONS:");
    println!("    -d <Duration>\texample: 1:15:30 'hours:minutes:seconds'");
    println!("    -t <Time>\t\texample: 5:30:pm");
    println!("    -i\t\t\tRuns indefinitely");
    println!();
    println!("EXAMPLES:");
    println!("    away -d 1:30:00");
    println!("    away -t 5:30:pm");
    println!("    away -i");
    println!();
}

fn display_version() {
    println!("v: {}", VERSION);
}