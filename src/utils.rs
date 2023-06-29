use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use openssl::base64;
use reqwest::{header::{HeaderMap, HeaderValue}, Response};
use rocket::{response::status::Custom, serde::json::Json, http::Status};
use scraper::{Selector, Html, ElementRef};
use url::form_urlencoded::Parse;

use crate::{
    types::{ErrorResponse, ResponseErrorAt, ErrorReturn},
    http::{APIPaths, HTTPErrorReturn, HTMLRespond, http_get_html, http_get},
    error::HTTPError
};

lazy_static! {
    static ref NOT_LOGIN_SELECTOR: Selector = Selector::parse("body > div").unwrap();
}

pub const HELLO_MESSAGE: &str = "Hello from HLHSInfo Server!";

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn get_timestamp_millisec() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn vecu8_to_hex_string(buffer: &[u8]) -> String {
    buffer
        .to_ascii_lowercase()
        .iter()
        .map(|bt| format!("{:02x}", bt))
        .collect::<Vec<String>>()
        .join("")
}

pub fn get_asp_cookie(string: &str) -> &str {
    string.split("; ").next().unwrap()
}

pub fn get_time_after(minute: u64) -> u64 {
    get_timestamp() + 60 * minute
}

pub fn get_time_after_millisec(minute: u128) -> u128 {
    get_timestamp_millisec() + 60000 * minute
}

pub fn error_message(path: &str, code: Status, message: &str, at: Option<&str>) -> ErrorReturn {
    let wrong = match at {
        Some(at) => Some(ResponseErrorAt {
            api: Some(String::from(path)),
            trace: None,
            at: Some(String::from(at))
        }),
        None => None
    };

    Custom(code, Json(ErrorResponse {
        message: String::from(message),
        timestamp: get_timestamp_millisec(),
        wrong
    }))
}

pub fn combine_path(host: &str, path: &str) -> String {
    format!("{}{}", host, path)
}

pub fn combine_page_path(host: &str, path: APIPaths) -> String {
    combine_path(host, path.path())
}

pub fn create_auth_header(cookie: &str) -> HeaderMap {
    let mut m = HeaderMap::new();
    m.append("Cookie", HeaderValue::from_str(cookie).unwrap());

    m
}

pub fn generate_http_error(path: &str, err: HTTPErrorReturn) -> ErrorReturn {
    match err {
        HTTPErrorReturn::RequestError(_) => return error_message(path, Status::ServiceUnavailable, HTTPError::RemoteServiceUnavailable.message(), Some("Remote server")),
        HTTPErrorReturn::StatusCodeError(_) => return error_message(path, Status::BadGateway, HTTPError::ServerError.message(), Some("Return status code"))
    }
}

pub fn generate_session_expire_error(path: &str) -> ErrorReturn {
    error_message(path, Status::Forbidden, HTTPError::SessionExpired.message(), None)
}

pub fn buffer_to_base64(input: &[u8]) -> String {
	base64::encode_block(input)
}

pub fn is_document_logined(document: &Html) -> bool {
    let doc = document.select(&NOT_LOGIN_SELECTOR).next();
    
    if doc.is_none() {
        return true
    }

    !doc.unwrap().text().collect::<Vec<_>>().contains(&"未登入")
}

pub fn convert_string_to_u32(string: &str) -> u32 {
    match string.parse::<u32>() {
        Ok(int) => int,
        Err(_) => 0
    }
}

pub fn html_to_text(element: ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<_>>()
        .join("")
}

pub fn find_string_in_url(parse_url: &Parse<'_>, param: &str) -> String {
    let find = parse_url.clone().find(|(key, _)| key == param);
    
    match find {
        Some(data) => data.1.to_string(),
        None => "".to_owned()
    }
}

pub async fn http_get_html_err_handle(api: &str, url: &str, headers: Option<HeaderMap>) -> Result<HTMLRespond, ErrorReturn> {
    match http_get_html(url, headers).await {
        Ok(res) => Ok(res),
        Err(err) => Err(generate_http_error(api, err))
    }
}

pub async fn http_get_err_handle(api: &str, url: &str, headers: Option<HeaderMap>) -> Result<Response, ErrorReturn> {
    match http_get(url, headers).await {
        Ok(res) => Ok(res),
        Err(err) => Err(generate_http_error(api, err))
    }
}