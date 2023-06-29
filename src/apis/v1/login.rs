use lazy_static::lazy_static;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::{
    types::{APIResponseJSON, Login, ErrorReturn, AuthToken, LoginInfoAuthToken},
    request_handler::{IncomingDataWrapper, decode_incoming, IncomingError, AuthorizationToken},
    utils::{self, create_auth_header, get_timestamp, get_time_after, generate_http_error},
    secure::sign,
    http::{http_post, APIPaths},
    apis::v1::get_user_info_short::get_user_info_short,
    config::read_config
};

lazy_static! {
    static ref EXPIRED_TIME: u64 = read_config().login_status_expired.into();
}

const API_PATH: &str = "/v1/login";
fn error_message(code: Status, message: &str, at: Option<&str>) -> ErrorReturn {
    utils::error_message(API_PATH, code, message, at)
}

#[derive(Debug, FromForm, Deserialize)]
pub struct IncomingData {
    username: String,
    password: String,
    vcode: String
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
struct DataPOST {
    __RequestVerificationToken: String,
    division: String,
    Loginid: String,
    LoginPwd: String,
    Uid: String,
    vcode: String
}

#[post("/login", data = "<data>")]
pub async fn api(auth: AuthorizationToken<LoginInfoAuthToken>, data: Result<IncomingDataWrapper<IncomingData>, IncomingError>) -> APIResponseJSON<Login> {
    let token = auth.0.clone();
    let data = match data {
        Ok(d) => decode_incoming(d),
        Err(err) => match err {
            IncomingError::MissingArguments(args) => {
                return Err(error_message(Status::BadRequest, "Argument is not satisfied", Some(&format!("Argument: {}", args.join(", ")))))
            },
            _ => return Err(error_message(Status::BadRequest, "Content-Type is not provided", Some("Header: Content-Type")))
        }
    };

    let page = utils::combine_page_path(&token.host, APIPaths::Login);
    let form = DataPOST {
        __RequestVerificationToken: token.site_key,
        division: "senior".to_owned(),
        Loginid: data.username,
        LoginPwd: data.password,
        Uid: "".to_owned(),
        vcode: data.vcode
    };

    let request = match http_post(&page, form, Some(create_auth_header(&token.cookie))).await {
        Ok(response) => response,
        Err(err) => return Err(generate_http_error(API_PATH, err))
    };

    let is_redict = request.status().is_redirection();

    if is_redict {
        let user_data = get_user_info_short(auth.0.clone()).await;

        if let Ok(data) = user_data {
            let token = sign(&AuthToken {
                host: token.host,
                cookie: token.cookie,
                user_data: data,

                iat: get_timestamp(),
                exp: get_time_after(*EXPIRED_TIME)
            }).unwrap();

            return Ok(Custom(Status::Ok, Json(Login {
                message: "Login successful!".to_owned(),
                authtoken: token
            })))
        }
    }

    Err(error_message(Status::Forbidden, "Login failed", None))
}