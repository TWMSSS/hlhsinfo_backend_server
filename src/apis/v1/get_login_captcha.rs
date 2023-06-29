use rocket::http::ContentType;

use crate::{
    request_handler::AuthorizationToken,
    types::{HTTPResponse, LoginInfoAuthToken},
    utils::{self, create_auth_header, http_get_err_handle},
    http::APIPaths,
    responder::FileResponse
};

const API_PATH: &str = "/v1/getLoginCaptcha";

#[get("/getLoginCaptcha")]
pub async fn api(auth: AuthorizationToken<LoginInfoAuthToken>) -> HTTPResponse<FileResponse> {
    let token = auth.0;
    let page = utils::combine_page_path(&token.host, APIPaths::LoginCaptcha);

    let captcha: Vec<u8> = http_get_err_handle(API_PATH, &page, Some(create_auth_header(&token.cookie))).await?
        .bytes()
        .await
        .unwrap()
        .into_iter()
        .collect();

    Ok(FileResponse {
        content_type: ContentType::GIF,
        file: captcha
    })
}