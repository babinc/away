use clap::{App, AppSettings, SubCommand, Arg};
use chrono::{NaiveTime, Utc, Timelike, Duration, Local};
use std::error::Error;
use device_query::{DeviceState, Keycode, DeviceQuery};
use std::{thread, io};
use inputbot::KeybdKey::{ScrollLockKey};
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    const DURATION: &str = "Duration";
    const TIME: &str = "Time";
    const INDEFINITELY: &str = "-i";

    let matches = App::new("away")
        .version("0.1.0")
        .about("Away management  tool")
        .setting(AppSettings::ColorAlways)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name(DURATION)
            .short("d")
            .long("duration")
            .help("example: 1:15:30 'hours:minutes:seconds'")
            .takes_value(true))
        .arg(Arg::with_name(TIME)
            .short("t")
            .long("time")
            .help("example: 5:30pm")
            .takes_value(true))
        .subcommand(SubCommand::with_name(INDEFINITELY)
            .about("Runs indefinitely"))
    .get_matches();

    if let Some(arg) = matches.value_of(DURATION) {
        run_duration(arg)?
    }

    if let Some(arg) = matches.value_of(TIME) {
        run_till_time(arg)?
    }

    if let Some(_) = matches.subcommand_matches(INDEFINITELY) {
        run_indefinitely();
    }

    println!("Welcome back 🙂");

    Ok(())
}

fn run_duration(arg: &str) -> Result<(), Box<dyn Error>> {
    let parsed_time: NaiveTime = match NaiveTime::parse_from_str(arg, "%H:%M:%S") {
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

fn run_till_time(arg: &str) -> Result<(), Box<dyn Error>> {
    let parsed_time: NaiveTime = match NaiveTime::parse_from_str(arg.to_lowercase().as_str(), "%I:%M:%p") {
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
