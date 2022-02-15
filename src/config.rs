use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::Path;
use directories::ProjectDirs;
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
        let file_name = "config.json";

        let proj_dirs = ProjectDirs::from("com", "CJB_Software",  "away")
            .ok_or(std::io::Error::new(ErrorKind::Other, "Error getting project directory"))?;

        let data_local = proj_dirs.data_local_dir().to_str()
            .ok_or(std::io::Error::new(ErrorKind::Other, "Error getting project directory as string"))?;

        if Path::new(data_local).is_dir() == false {
            fs::create_dir_all(data_local)?;
            println!("Directory created: {}", data_local)
        }

        let full_path = format!("{}\\{}", data_local, file_name);
        let mut config = Config::default();

        let path = Path::new(full_path.as_str());

        if path.exists() && path.is_file() {
            let file_str = fs::read_to_string(&full_path)?;

            if file_str.len() > 0 {
                match serde_json::from_str(file_str.as_ref()) {
                    Ok(res) => config = res,
                    Err(err) => {
                        let error_msg = format!("Path: {}, Error parsing config.json: {}. Applying default values", path.to_str().unwrap(), err);
                        eprintln!("{}", error_msg);
                        fs::remove_file(&full_path)?;
                        return Ok(Config::default());
                    }
                };
            }

            println!("Using config file: {}", full_path);
        }
        else {
            let mut file = match File::create(&full_path) {
                Ok(res) => res,
                Err(err) => {
                    let error_msg = format!("Error: Trying parse new config.json: {}", err.to_string());
                    let err = std::io::Error::new(ErrorKind::Other, error_msg);
                    return Err(Box::new(err));
                }
            };

            let config_json = serde_json::to_string_pretty(&config)?;

            if config_json.len() > 0 {
                match file.write_all(config_json.as_ref()) {
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

        let output = serde_json::to_string_pretty(&config)?;
        println!("config values: {}", output);

        Ok(config)
    }
}

