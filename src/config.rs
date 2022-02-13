use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub user_input_wait_time_ms: u64,
    pub key_press_time_ms: u64,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            user_input_wait_time_ms: 60_000,
            key_press_time_ms: 100
        }
    }
}
