use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::Path;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub user_input_wait_time_ms: u64,
    pub key_press_time_ms: u64,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            user_input_wait_time_ms: 60_000,
            key_press_time_ms: 100
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Config, Box<dyn Error>> {
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
                        return Ok(Config::default());
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
}

