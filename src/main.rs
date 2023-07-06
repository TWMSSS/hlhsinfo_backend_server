#[macro_use] extern crate rocket;

use std::{path::Path, fs::create_dir_all, env::consts::{OS, ARCH}};

use hlhsinfo_backend_server::{config::read_config, routes::create_server, utils::DEFAULT_FILE_PATH};
use rocket::{config::Config, log::LogLevel};

#[launch]
fn rocket() -> _ {
    if !Path::new(&*DEFAULT_FILE_PATH).exists() {
        create_dir_all(&*DEFAULT_FILE_PATH).expect("Cannot create directory for config file");
    }

    let global_config = read_config();

    println!("{}", "=".repeat(20));
    println!();
    println!("HLHSInfo Backend Server");
    println!();
    println!("Listening at {}", global_config.port);
    println!();
    println!("Running at {}({})", OS, ARCH);
    println!("File storage at {}", &*DEFAULT_FILE_PATH);
    println!();
    println!("{}", "=".repeat(20));

    let config = Config::figment()
        .merge(("port", global_config.port))
        .merge(("ident", "HLHSInfo"))
        .merge(("log_level", LogLevel::Off));

    create_server(config)
}