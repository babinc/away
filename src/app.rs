use std::{process, thread};
use std::sync::mpsc::{self, Sender};
use chrono::{Duration, Local, NaiveTime, Timelike, Utc};
use device_query::{DeviceQuery, DeviceState, Keycode};
use hhmmss::Hhmmss;
use inputbot::KeybdKey::ScrollLockKey;
use mouse_rs::Mouse;
use mouse_rs::types::Point;
use crate::Config;
use crate::spinner::Spinner;
use crate::ui::Ui;
use anyhow::{Result, anyhow};

pub struct App {
    config: Config,
    ui: Ui,
    spinner: Spinner,
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            config,
            ui: Ui::new(),
            spinner: Spinner::new(),
        }
    }

    pub fn run_till_time(&mut self, arg: &Vec<String>) -> Result<()> {
        let parsed_time: NaiveTime = match NaiveTime::parse_from_str(arg[2].to_lowercase().as_str(), "%I:%M:%p") {
            Ok(res) => res,
            Err(err) => return Err(anyhow!("Could not parse time input. Error: {},  Example: 10:00:am", err.to_string()))
        };

        let (user_activity_timeout_tx, user_activity_timeout_rx) = mpsc::channel();
        let (user_activity_check_tx, user_activity_check_rx) = mpsc::channel();
        let mut is_waiting_for_timeout = false;
        let mut has_user_activity = false;

        self.check_for_user_activity(&user_activity_check_tx);

        Self::check_for_exit_key();

        loop {
            if user_activity_check_rx.try_recv().is_ok() {
                has_user_activity = true;
            }

            if has_user_activity && is_waiting_for_timeout == false {
                is_waiting_for_timeout = true;
                has_user_activity = false;
                self.user_activity_wait(&user_activity_timeout_tx);
            }

            if user_activity_timeout_rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            let local_time = Local::now().time();
            let elapsed_time = parsed_time - local_time;

            if !is_waiting_for_timeout {
                let time_output = elapsed_time.hhmmss();
                self.ui.write(time_output.as_str());
                self.stay_awake();
            }

            if local_time >= parsed_time.to_owned() {
                break;
            }
        }

        Ok(())
    }

    pub fn run_duration(&mut self, arg: &Vec<String>) -> Result<()> {
        let parsed_time: NaiveTime = match NaiveTime::parse_from_str(arg[2].as_str(), "%H:%M:%S") {
            Ok(res) => res,
            Err(err) => return Err(anyhow!("Could not parse duration input. Error: {}, Example: 1:15:30", err.to_string()))
        };

        let hour_dur = Duration::hours(parsed_time.hour() as i64);
        let min_dur = Duration::minutes(parsed_time.minute() as i64);
        let sec_dur = Duration::seconds(parsed_time.second() as i64);

        let duration = hour_dur + min_dur + sec_dur;

        let mut now = Utc::now();
        let stop_time = now + duration;

        let (user_activity_timeout_tx, user_activity_timeout_rx) = mpsc::channel();
        let (user_activity_check_tx, user_activity_check_rx) = mpsc::channel();
        let mut is_waiting_for_timeout = false;
        let mut has_user_activity = false;

        self.check_for_user_activity(&user_activity_check_tx);

        Self::check_for_exit_key();

        loop {
            if user_activity_check_rx.try_recv().is_ok() {
                has_user_activity = true;
            }

            if has_user_activity && is_waiting_for_timeout == false {
                is_waiting_for_timeout = true;
                has_user_activity = false;
                self.user_activity_wait(&user_activity_timeout_tx);
            }

            if user_activity_timeout_rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            if !is_waiting_for_timeout {
                now = Utc::now();
                let elapsed_time = stop_time - now;

                let time_output = elapsed_time.hhmmss();

                self.ui.write(time_output.as_str());
                self.stay_awake();
            }

            if now >= stop_time {
                break;
            }
        }

        Ok(())
    }

    pub fn run_indefinitely(&mut self) {
        let (user_activity_timeout_tx, user_activity_timeout_rx) = mpsc::channel();
        let (user_activity_check_tx, user_activity_check_rx) = mpsc::channel();
        let mut is_waiting_for_timeout = false;
        let mut has_user_activity = false;

        self.check_for_user_activity(&user_activity_check_tx);

        Self::check_for_exit_key();

        loop {
            if user_activity_check_rx.try_recv().is_ok() {
                has_user_activity = true;
            }

            if has_user_activity && is_waiting_for_timeout == false {
                is_waiting_for_timeout = true;
                has_user_activity = false;
                self.user_activity_wait(&user_activity_timeout_tx);
            }

            if user_activity_timeout_rx.try_recv().is_ok() {
                is_waiting_for_timeout = false;
            }

            if !is_waiting_for_timeout {
                let output = format!("Staying Awake: {} ", self.spinner.next_char());
                self.ui.write(output.as_str());
                self.stay_awake();
            }
        }
    }

    fn stay_awake(&self) {
        ScrollLockKey.press();
        thread::sleep(std::time::Duration::from_millis(self.config.key_press_time_ms));
        ScrollLockKey.release();
    }

    fn check_for_exit_key() {
        thread::spawn(|| {
            loop {
                let keys: Vec<Keycode> = DeviceState.get_keys();

                let ctrl_pressed = keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl);
                let q_pressed = keys.contains(&Keycode::Q);

                if ctrl_pressed && q_pressed {
                    process::exit(0);
                }

                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }

    fn user_activity_wait(&mut self, tx: &Sender<()>) {
        self.ui.write("User Activity Detected");
        let thread_tx = tx.clone();
        let user_wait_time = self.config.user_input_wait_time_ms;
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(user_wait_time));
            thread_tx.send(()).expect("Error sending cross thread user activity wait is done")
        });
    }

    fn check_for_user_activity(&self, tx: &Sender<()>) {
        let thread_tx = tx.clone();
        let user_wait_time = self.config.user_input_wait_time_ms;
        let mouse = Mouse::new();
        let mut last_mouse_pos = mouse.get_position().unwrap_or(Point { x: 0, y: 0 });
        thread::spawn(move || {
            loop {
                let has_keyboard_input = DeviceState.get_keys().len() > 0;
                let current_mouse_pos = mouse.get_position().unwrap_or(Point { x: 0, y: 0 });
                let has_mouse_input = Self::are_points_equal(&current_mouse_pos, &last_mouse_pos) == false;
                if has_keyboard_input || has_mouse_input {
                    thread_tx.send(()).expect("Error sending cross thread User Input Detection");
                    thread::sleep(std::time::Duration::from_millis(user_wait_time));
                }
                last_mouse_pos = current_mouse_pos;
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }

    fn are_points_equal(left: &Point, right: &Point) -> bool {
        if left.x == right.x && left.y == right.y {
            true
        }
        else {
            false
        }
    }
}