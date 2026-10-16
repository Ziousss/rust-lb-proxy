use std::fs::read_to_string;

use crate::structs::Config;

pub fn read_config(path: &str) -> Config {
    let text = read_to_string(path).expect("Unable to read config file\n");

    // defaults, used if missing file
    let mut config = Config {
        bind: String::from("10.0.0.20:8080"),
        backends: Vec::new(),
        connect_timeout_secs: 5,
        check_interval_secs: 5,
    };

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        if let Some((key, value)) = line.split_once("=") {
            let key = key.trim();
            let value = value.trim();

            if key == "bind" {
                config.bind = value.to_string();
            } else if key == "backends" {
                for part in value.split(",") {
                    config.backends.push(part.trim().to_string());
                }
            } else if key == "connection_timeout" {
                config.connect_timeout_secs = value
                    .parse()
                    .expect("Unable to parse connect timeout from config");
            } else if key == "check_sleep" {
                config.check_interval_secs = value
                    .parse()
                    .expect("Unable to parse check interval from config");
            } else {
                eprintln!("Unknown key in config parsing: {}", key);
            }
        }
    }

    config
}
