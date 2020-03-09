use chrono::{NaiveTime, Utc, Timelike, Duration, Local};
use std::error::Error;
use device_query::{DeviceState, Keycode, DeviceQuery};
use std::{thread, io, env};
use inputbot::KeybdKey::{ScrollLockKey};
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    parse_arguments(&args)?;

    Ok(())
}

fn parse_arguments(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    const TIME_ARGUMENT: &str = "-t";
    const DURATION: &str = "-d";
    const INDEFINITELY: &str = "-i";
    const HELP: &str = "-h";
    const VERSION: &str = "-v";

    if args.contains(&TIME_ARGUMENT.to_string()) == true {
        let index = args.iter().position(|r| r == &TIME_ARGUMENT.to_string()).unwrap();
        if args.len() > index + 1 {
            run_till_time(&args)?
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
            run_duration(&args)?
        }
        else {
            eprintln!("Error: missing duration parameter");
            println!();
            display_usage();
        }
    }
    else if args.contains(&INDEFINITELY.to_string()) == true {
        run_indefinitely();
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

fn run_duration(arg: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let parsed_time: NaiveTime = match NaiveTime::parse_from_str(arg[2].as_str(), "%H:%M:%S") {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Could not parse duration input. Example: 1:15:30");
            return Err(Box::new(err));
        }
    };

    let hour_dur = Duration::hours(parsed_time.hour() as i64);
    let min_dur = Duration::minutes(parsed_time.minute() as i64);
    let sec_dur = Duration::seconds(parsed_time.second() as i64);

    let duration = hour_dur + min_dur + sec_dur;

    let now = Utc::now();
    let stop_time = now + duration;

    loop {
        let exited = check_for_exit_key();

        let now = Utc::now();
        let elapsed_time = stop_time - now;

        print!("{}:{}:{}", elapsed_time.num_seconds() / 3600, (elapsed_time.num_seconds() / 60) % 60, elapsed_time.num_seconds() % 60);
        io::stdout().flush().unwrap();

        stay_awake();

        print!("\r");
        io::stdout().flush().unwrap();

        if exited || now >= stop_time {
            break;
        }
    }

    Ok(())
}

fn run_till_time(arg: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let parsed_time: NaiveTime = match NaiveTime::parse_from_str(arg[2].to_lowercase().as_str(), "%I:%M:%p") {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Could not parse time input. Example: 10:00:am");
            return Err(Box::new(err));
        }
    };

    loop {
        let exited = check_for_exit_key();

        let local_time = Local::now().time();
        let elapsed_time = parsed_time - local_time;

        print!("{}:{}:{}", elapsed_time.num_seconds() / 3600, (elapsed_time.num_seconds() / 60) % 60, elapsed_time.num_seconds() % 60);
        io::stdout().flush().unwrap();

        stay_awake();

        print!("\r");
        io::stdout().flush().unwrap();

        if exited || local_time >= parsed_time.to_owned() {
            break;
        }
    }

    Ok(())
}

fn run_indefinitely() {
    loop {
        let exited = check_for_exit_key();

        stay_awake();

        if exited {
            break;
        }
    }
}

fn stay_awake() {
    ScrollLockKey.press();
    thread::sleep(std::time::Duration::from_millis(150));
    ScrollLockKey.release();
}

fn check_for_exit_key() -> bool {
    let keys: Vec<Keycode> = DeviceState.get_keys();
    for key in keys.iter() {
        return if *key == Keycode::Q {
            true
        } else {
            false
        }
    }

    return false;
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
