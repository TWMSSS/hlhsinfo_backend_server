use std::{fs::{File, create_dir_all}, path::Path, sync::Mutex, env::{consts::OS, self}};
use lazy_static::lazy_static;
use serde_yaml::{self};

use crate::types::Config;

const CONFIG_FILE: &str = "config.yaml";

lazy_static! {
    static ref CONFIG_FILE_PATH: String = {
        match OS {
            "linux" => "/usr/etc/hlhsinfo_backend_server".to_owned(),
            "windows" => {
                match env::var("USERPROFILE") {
                    Ok(profile) => format!("{}/{}", profile, r#"\AppData\Roaming\hlhsinfo_backend_server"#),
                    Err(_) => "".to_owned()
                }
            },
            _ => "".to_owned()
        }
    };
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: String::from("HLHSInfo Open Source"),
            port: 1156,
            share_expired: 1800000,
            logininfo_expired: 5,
            login_status_expired: 60,
            failed_expried: 3600000,
            failed_times_lock: 5,
            cache_enabled: true,
            cache_expired: 48,
            check_cycle: 5,
            enable_record: true
         }
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

pub fn read_config() -> Config {
    if let Some(config) = CONFIG.lock().unwrap().as_ref() {
        return config.clone();
    }

    let path = format!("{}/{}", *CONFIG_FILE_PATH, CONFIG_FILE);

    if !Path::exists(Path::new(&path)) {
        create_dir_all(&*CONFIG_FILE_PATH).expect("Cannot create directory for config file");

        let file = File::create(&path).expect("Cannot create config file.");
        let default_config: Config = Default::default();

        serde_yaml::to_writer(file, &default_config).expect("Cannot write default config.");
    }

    let file = File::open(&path).expect("Cannot read config file.");
    let config: Config = serde_yaml::from_reader(file).expect("Cannot read config values.");

    *CONFIG.lock().unwrap() = Some(config.clone());

    config
}