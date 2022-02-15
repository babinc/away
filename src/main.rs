mod app;
mod spinner;
mod ui;
mod config;

use std::error::Error;
use std::{env, fs};
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::Path;
use crate::app::App;
use crate::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    let args: Vec<String> = env::args().collect();

    parse_arguments(&args, config)?;

    Ok(())
}

fn parse_arguments(args: &Vec<String>, config: Config) -> Result<(), Box<dyn Error>> {
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
    let version = env!("CARGO_PKG_VERSION");
    println!("v: {}", version);
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let file_path = "config.json";

    let mut config = Config::default();

    let path = Path::new(file_path);

    let exe_path = match env::current_exe() {
        Ok(exe_path) => format!("{}", exe_path.display()),
        Err(e) => {
            let error_msg = format!("Error getting executable path: {}", e.to_string());
            let err = std::io::Error::new(ErrorKind::Other, error_msg);
            return Err(Box::new(err));
        },
    };

    let full_path = format!("{}\\{}", exe_path, path.to_str().unwrap());

    if path.exists() && path.is_file() {
        let file_str = fs::read_to_string(file_path)?;

        if file_str.len() > 0 {
            match serde_json::from_str(file_str.as_ref()) {
                Ok(res) => config = res,
                Err(err) => {
                    let error_msg = format!("Path: {}, Error parsing config.json: {}. Applying default values", path.to_str().unwrap(), err);
                    eprintln!("{}", error_msg);
                    fs::remove_file(file_path)?;
                    return Ok(config::Config::default());
                }
            };
        }

        println!("Using config file: {}", full_path);
    }
    else {
        let mut file = match File::create(&file_path) {
            Ok(res) => res,
            Err(err) => {
                let error_msg = format!("Error: Trying parse new config.json: {}", err.to_string());
                let err = std::io::Error::new(ErrorKind::Other, error_msg);
                return Err(Box::new(err));
            }
        };

        let json_data = serde_json::to_string_pretty(&config)?;

        if json_data.len() > 0 {
            match file.write_all(json_data.as_ref()) {
                Ok(_res) => {},
                Err(err) => {
                    let error_msg = format!("Error trying to write new config.json: {}", err.to_string());
                    let err = std::io::Error::new(ErrorKind::Other, error_msg);
                    return Err(Box::new(err));
                }
            }
        }

        println!("New config file created: {}", full_path);
    }

    println!("config values: {:?}", config);

    Ok(config)
}