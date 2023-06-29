use lazy_static::lazy_static;
use rocket::{serde::json::Json, http::Status, response::status::Custom};
use scraper::Selector;
use reqwest::{header::SET_COOKIE, StatusCode};

use crate::{
    types::{LoginInfo, APIResponseJSON, LoginInfoAuthToken, ErrorReturn},
    config::read_config,
    utils::{get_timestamp, self, get_time_after},
    error::HTTPError,
    http::{http_get_html, HTTPErrorReturn},
    secure::sign
};

lazy_static! {
    static ref CHECK_SELECTOR: Selector = Selector::parse("meta[name=keywords]").unwrap();
    static ref VERIFY_CODE_SELECTOR: Selector = Selector::parse("input[name=__RequestVerificationToken]").unwrap();
    static ref CAPTCHA_CHECK_SELECTOR: Selector = Selector::parse("img#imgvcode").unwrap();
    static ref EXPIRED_TIME: u64 = read_config().logininfo_expired.into();
}

const API_PATH: &str = "/v1/getLoginInfo";
fn error_message(code: Status, message: &str, at: &str) -> ErrorReturn {
    utils::error_message(API_PATH, code, message, Some(at))
}

#[get("/getLoginInfo?<host>")]
pub async fn api(host: Option<&str>) -> APIResponseJSON<LoginInfo> {
    let host = match host {
        Some(x) => x,
        None => return Err(error_message(Status::BadRequest, "Wrong arguments", "Argument: host"))
    };

    let hst = match url::Url::parse(host) {
        Ok(r) => {
            let hos = r.host().unwrap();
            format!("{}://{}/online/", r.scheme(), hos)
        },
        Err(_) => return Err(error_message(Status::InternalServerError, HTTPError::ServerError.message(), "Parsing host url"))
    };

    let respond = match http_get_html(&hst, None).await {
        Ok(v) => v,
        Err(err) => return Err(match err {
            HTTPErrorReturn::RequestError(_) => error_message(Status::ServiceUnavailable, HTTPError::RemoteServiceUnavailable.message(), "Remote server"),
            HTTPErrorReturn::StatusCodeError(code) => match code {
                StatusCode::NOT_FOUND => error_message(Status::BadRequest, HTTPError::NotAValidHost.message(), "Argument: host"),
                _ => error_message(Status::BadGateway, HTTPError::ServerError.message(), "Return status code")
            }
        })
    };

    let cookie = match respond.header.get(SET_COOKIE) {
        Some(cookie) => utils::get_asp_cookie(cookie.to_str().unwrap()),
        None => return Err(error_message(Status::ServiceUnavailable, HTTPError::RemoteServiceUnavailable.message(), "Remote server"))
    };

    let r = match respond.html.select(&CHECK_SELECTOR).next() {
        Some(ele) => {
            let v = ele.value().attr("content").unwrap();
            String::from(v).eq(&"欣河資訊") 
        },
        None => return Err(error_message(Status::BadRequest, HTTPError::NotAValidHost.message(), "Argument: host"))
    };
    
    if !r {
        return Err(error_message(Status::BadRequest, HTTPError::NotAValidHost.message(), "Argument: host"));
    }

    let auth_code = respond.html.select(&VERIFY_CODE_SELECTOR).next().unwrap().value().attr("value").unwrap().to_string();
    let is_captcha_needed = match respond.html.select(&CAPTCHA_CHECK_SELECTOR).next() {
        Some(_) => true,
        None =>  false
    };

    let token = sign(&LoginInfoAuthToken {
        host: hst,
        site_key: auth_code,
        cookie: cookie.to_owned(),
        need_captcha: is_captcha_needed,

        iat: get_timestamp(),
        exp: get_time_after(*EXPIRED_TIME)
    }).unwrap();

    Ok(Custom(Status::Ok, Json(LoginInfo {
        authToken: token,
        need_captcha: is_captcha_needed
    })))
}