use rocket::{Rocket, Build};

pub mod v1;

pub fn init_api_routes(server: Rocket<Build>) -> Rocket<Build> {
    v1::init_v1_api(server)
}