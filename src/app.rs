use std::thread;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use chrono::{Duration, Local, NaiveTime, Timelike, Utc};
use device_query::{DeviceQuery, DeviceState, Keycode};
use hhmmss::Hhmmss;
use inputbot::KeybdKey::ScrollLockKey;
use crate::Config;
use crate::spinner::Spinner;
use crate::ui::Ui;

pub struct App {
    config: Config,
    ui: Ui,
    spinner: Spinner
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            config,
            ui: Ui::new(),
            spinner: Spinner::new()
        }
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
                self.user_activity_wait(&tx);
            }

            if rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            let local_time = Local::now().time();
            let elapsed_time = parsed_time - local_time;

            if !is_waiting_for_timeout {
                let time_output = elapsed_time.hhmmss();
                self.ui.write(time_output.as_str());
                self.stay_awake();
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
                self.user_activity_wait(&tx);
            }

            if rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            if !is_waiting_for_timeout {
                now = Utc::now();
                let elapsed_time = stop_time - now;

                let time_output = elapsed_time.hhmmss();

                self.ui.write(time_output.as_str());
                self.stay_awake();
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

        loop {
            let exited = Self::check_for_exit_key();

            let has_user_activity = Self::check_for_user_activity();

            if has_user_activity && is_waiting_for_timeout == false {
                is_waiting_for_timeout = true;
                self.user_activity_wait(&tx);
            }

            if rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            if !is_waiting_for_timeout {
                let output = format!("Staying Awake: {} ", self.spinner.next_char());
                self.ui.write(output.as_str());
                self.stay_awake();
            }

            if exited {
                break;
            }
        }
    }

    fn stay_awake(&self) {
        ScrollLockKey.press();
        thread::sleep(std::time::Duration::from_millis(self.config.key_press_time_ms));
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

    fn user_activity_wait(&mut self, tx: &Sender<()>) {
        self.ui.write("User Activity Detected");
        let thread_tx = tx.clone();
        let user_wait_time = self.config.user_input_wait_time_ms;
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(user_wait_time));
            thread_tx.send(()).unwrap();
        });
    }

    fn check_for_user_activity() -> bool {
        return DeviceState.get_keys().len() > 0
    }
}