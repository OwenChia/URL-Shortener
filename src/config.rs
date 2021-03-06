#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub redis_connection: String,
    pub url_length: Option<usize>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            redis_connection: "redis+unix:/run/redis/redis.sock".to_owned(),
            url_length: Some(8),
        }
    }

    pub fn from_file(path: &Path) -> Config {
        if !path.exists() {
            panic!("config file does not exists, eg. `config.toml`.")
        }

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open config file: {}", why.description()),
        };

        let mut toml_str = String::new();
        if let Err(why) = file.read_to_string(&mut toml_str) {
            panic!("couldn't open config file: {}", why.description());
        }

        let config: Config = match toml::from_str(&toml_str) {
            Ok(config) => config,
            Err(why) => panic!("couldn't parse config: {}", why.description()),
        };

        config
    }
}
