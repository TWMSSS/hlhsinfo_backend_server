use lazy_static::lazy_static;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use scraper::Selector;
use tokio::join;

use crate::{
    request_handler::AuthorizationToken,
    utils::{combine_page_path, create_auth_header},
    types::{UserProfileShortValue, APIResponseJSON, UserProfileShortData, AuthToken, LoginInfoAuthToken},
    http::{APIPaths, http_get_html},
    error::FetchError
};

lazy_static! {
    static ref USER_DATA: Selector = Selector::parse("#authirty1 > td").unwrap();
    static ref CLASS_DATA: Selector = Selector::parse("td").unwrap();
}

struct UserInfo {
    class_number: String,
    school_number: String,
    user_name: String,
    gender: String
}

async fn fetch_class(host: &str, cookie: &str) -> Result<String, FetchError> {
    let url = combine_page_path(host, APIPaths::ClassData);
    let data = match http_get_html(&url, Some(create_auth_header(cookie))).await {
        Ok(r) => r.html,
        Err(_) => return Err(FetchError::FetchFailed)
    };

    if let Some(ele) = data.select(&CLASS_DATA).next() {
        let s = ele
            .text()
            .collect::<Vec<_>>()
            .join("")
            .replace(" ", "")
            .replace("\n\n\n", "")
            .split("：")
            .nth(1)
            .unwrap_or("")
            .split_terminator("\n")
            .nth(0)
            .unwrap_or("")
            .to_owned();
        return Ok(s)
    }

    Err(FetchError::FetchFailed)
}

async fn fetch_user(host: &str, cookie: &str) -> Result<UserInfo, FetchError> {
    let url = combine_page_path(host, APIPaths::ProfileShort);
    let data = match http_get_html(&url, Some(create_auth_header(cookie))).await {
        Ok(r) => r.html,
        Err(_) => return Err(FetchError::FetchFailed)
    };

    let mut data = data
        .select(&USER_DATA)
        .map(|data| {
            data
                .text()
                .collect::<Vec<_>>()
                .join("")
                .replace(" ", "")
                .replace("\n", "")
        });

    let output = UserInfo {
        class_number: data.next().unwrap(),
        school_number: data.next().unwrap(),
        user_name: data.next().unwrap(),
        gender: data.next().unwrap()
    };

    Ok(output)
}

async fn get_info_from_web(token: LoginInfoAuthToken) -> Result<UserProfileShortValue, FetchError> {
    let (class, user) = join!(fetch_class(&token.host, &token.cookie), fetch_user(&token.host, &token.cookie));

    match (class, user) {
        (Ok(class_data), Ok(user_data)) => return Ok(UserProfileShortValue {
            className: class_data,
            classNumber: user_data.class_number,
            gender: user_data.gender,
            schoolNumber: user_data.school_number,
            userName: user_data.user_name
        }),
        _ => return Err(FetchError::AuthError)
    }
}

pub async fn get_user_info_short(token: LoginInfoAuthToken) -> Result<UserProfileShortValue, FetchError> {
    Ok(get_info_from_web(token).await?)
}

#[get("/getUserInfoShort")]
pub async fn api(auth: AuthorizationToken<AuthToken>) -> APIResponseJSON<UserProfileShortData> {
    Ok(Custom(Status::Ok, Json(UserProfileShortData {
        message: "Get user profile short successful".to_owned(),
        data: auth.0.user_data
    })))
}