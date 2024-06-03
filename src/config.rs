use std::{fs::File, io::Write};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

const CONFIG_PATH: &str = "config.json";

lazy_static! {
    static ref CONFIG: Config = {
        let config_file = File::open(CONFIG_PATH);
        let config_file = match config_file {
            Ok(file) => file,
            Err(_) => {
                let default_file = File::create(CONFIG_PATH).unwrap();
                write!(
                    &default_file,
                    "{}",
                    to_string_pretty(&default_config()).unwrap()
                )
                .unwrap();
                File::open(CONFIG_PATH).unwrap()
            }
        };
        serde_json::from_reader(config_file).unwrap()
    };
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    server: ServerConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    address: String,
    port: String,
    cors_permissive: bool,
}

pub fn get_server_address_config() -> String {
    get_config().server.address.clone()
}

pub fn get_server_port_config() -> String {
    get_config().server.port.clone()
}

pub fn get_server_cors_config() -> bool {
    get_config().server.cors_permissive.clone()
}

fn get_config() -> &'static Config {
    &CONFIG
}

fn default_config() -> Config {
    Config {
        server: ServerConfig {
            address: String::from("127.0.0.1"),
            port: String::from("7001"),
            cors_permissive: false,
        },
    }
}
