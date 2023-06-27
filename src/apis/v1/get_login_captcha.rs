use rocket::http::ContentType;

use crate::{
    request_handler::AuthorizationToken,
    types::{HTTPResponse, LoginInfoAuthToken},
    utils::{self, create_auth_header, generate_http_error},
    http::{http_get, APIPaths},
    responder::FileResponse
};

const API_PATH: &str = "/v1/getLoginCaptcha";

#[get("/getLoginCaptcha")]
pub async fn api(auth: AuthorizationToken<LoginInfoAuthToken>) -> HTTPResponse<FileResponse> {
    let token = auth.0;
    let page = utils::combine_page_path(&token.host, APIPaths::LoginCaptcha);

    let captcha: Vec<u8> = match http_get(&page, Some(create_auth_header(&token.cookie))).await {
        Ok(res) => res.bytes().await.unwrap().into_iter().collect(),
        Err(err) => return Err(generate_http_error(API_PATH, err))
    };

    Ok(FileResponse {
        content_type: ContentType::GIF,
        file: captcha
    })
}