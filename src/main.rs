#[macro_use] extern crate rocket;

use hlhsinfo_backend_server::{config::read_config, routes::create_server};
use rocket::{config::Config, log::LogLevel};

#[launch]
fn rocket() -> _ {
    let global_config = read_config();

    let config = Config::figment()
        .merge(("port", global_config.port))
        .merge(("ident", "HLHSInfo"))
        .merge(("log_level", LogLevel::Critical));

    create_server(config)
}