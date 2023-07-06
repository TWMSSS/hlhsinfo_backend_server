use rocket::{Rocket, Build};

mod get_login_info;
mod get_login_captcha;
mod login;
mod get_user_info_short;
mod get_user_profile;
mod get_available_score;
mod get_reward_and_punish;
mod get_score;
mod get_lack;

pub fn init_v1_api(server: Rocket<Build>) -> Rocket<Build> {
    server.mount("/v1", routes![
        // Login data
        get_login_info::api,
        get_login_captcha::api,
        login::api,

        // User data
        get_user_info_short::api,
        get_user_profile::api,
        get_available_score::api,
        get_reward_and_punish::api,
        get_score::api,
        get_lack::api
    ])
}