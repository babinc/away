use std::{io, thread};
use std::error::Error;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use chrono::{Duration, Local, NaiveTime, Timelike, Utc};
use device_query::{DeviceQuery, DeviceState, Keycode};
use hhmmss::Hhmmss;
use inputbot::KeybdKey::ScrollLockKey;

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run_till_time(&mut self, arg: &Vec<String>) -> Result<(), Box<dyn Error>> {
        let parsed_time: NaiveTime = match NaiveTime::parse_from_str(arg[2].to_lowercase().as_str(), "%I:%M:%p") {
            Ok(res) => res,
            Err(err) => {
                eprintln!("Could not parse time input. Example: 10:00:am");
                return Err(Box::new(err));
            }
        };

        let (tx, rx) = mpsc::channel();
        let mut is_waiting_for_timeout = false;

        loop {
            let exited = Self::check_for_exit_key();

            let has_user_activity = Self::check_for_user_activity();

            if has_user_activity && is_waiting_for_timeout == false {
                is_waiting_for_timeout = true;
                Self::user_activity_wait(&tx);
            }

            if rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            let local_time = Local::now().time();
            let elapsed_time = parsed_time - local_time;

            if !is_waiting_for_timeout {
                let time_output = elapsed_time.hhmmss();
                Self::write_output(time_output.as_str());
                Self::stay_awake();
            }

            if exited || local_time >= parsed_time.to_owned() {
                break;
            }
        }

        Ok(())
    }

    pub fn run_duration(&mut self, arg: &Vec<String>) -> Result<(), Box<dyn Error>> {
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

        let mut now = Utc::now();
        let stop_time = now + duration;

        let (tx, rx) = mpsc::channel();
        let mut is_waiting_for_timeout = false;

        loop {
            let exited = Self::check_for_exit_key();

            let has_user_activity = Self::check_for_user_activity();

            if has_user_activity && is_waiting_for_timeout == false {
                is_waiting_for_timeout = true;
                Self::user_activity_wait(&tx);
            }

            if rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            if !is_waiting_for_timeout {
                now = Utc::now();
                let elapsed_time = stop_time - now;

                let time_output = elapsed_time.hhmmss();

                Self::write_output(time_output.as_str());
                Self::stay_awake();
            }

            if exited || now >= stop_time {
                break;
            }
        }

        Ok(())
    }

    pub fn run_indefinitely(&mut self) {
        let (tx, rx) = mpsc::channel();
        let mut is_waiting_for_timeout = false;

        let mut write_stay_awake = true;

        loop {
            let exited = Self::check_for_exit_key();

            let has_user_activity = Self::check_for_user_activity();

            if has_user_activity && is_waiting_for_timeout == false {
                is_waiting_for_timeout = true;
                Self::user_activity_wait(&tx);
            }

            if rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
                write_stay_awake = true;
            }

            if !is_waiting_for_timeout {
                if write_stay_awake {
                    Self::write_output("Staying Awake");
                    write_stay_awake = false;
                }
                Self::stay_awake();
            }

            if exited {
                break;
            }
        }
    }

    fn stay_awake() {
        ScrollLockKey.press();
        thread::sleep(std::time::Duration::from_millis(100));
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

    fn user_activity_wait(tx: &Sender<()>) {
        Self::write_output("User Activity Detected");
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(60));
            thread_tx.send(()).unwrap();
        });
    }

    fn write_output(output: &str) {
        print!("\r");
        io::stdout().flush().unwrap();
        print!("                                             ");
        io::stdout().flush().unwrap();
        print!("\r");
        io::stdout().flush().unwrap();
        print!("{}", output);
        io::stdout().flush().unwrap();
    }

    fn check_for_user_activity() -> bool {
        return DeviceState.get_keys().len() > 0
    }
}