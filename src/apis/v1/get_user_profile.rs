use lazy_static::lazy_static;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use scraper::{Selector, Html};
use tokio::join;
use url::Url;

use crate::{
    request_handler::AuthorizationToken,
    types::{APIResponseJSON, UserData, AuthToken, UserDataValues, UserCollect},
    http::{APIPaths, ReplaceString, http_get},
    utils::{combine_page_path, create_auth_header, generate_http_error, generate_session_expire_error, combine_path, buffer_to_base64, is_document_logined}
};

lazy_static! {
    static ref PROFILE_IMAGE_SELECTOR: Selector = Selector::parse("img").unwrap();
    static ref DATA_SELECTOR: Selector = Selector::parse("table[class='le_04 padding2 spacing2'] tr").unwrap();
    static ref FELIDS_SELECTOR: Selector = Selector::parse("td").unwrap();
}

const BASE64_IMAGE_HEAD: &str = "data:image/png;base64,";
const API_PATH: &str = "/v1/login";

async fn fetch_image(host: &str, cookie: &str, id: &str) -> String {
    let page = combine_path(host, &APIPaths::ProfileImage.replace(vec![ReplaceString {
        match_string: "$imgid$".to_owned(),
        replacement: id.to_owned()
    }]));

    let data = http_get(&page, Some(create_auth_header(cookie))).await.unwrap();
    let buffer = data.bytes().await.unwrap();

    buffer_to_base64(&buffer)
}

async fn get_image(token: AuthToken, document: &str) -> String {
    let image_path = {
        let doc = Html::parse_document(document);
        let image_select = doc.select(&PROFILE_IMAGE_SELECTOR).next().unwrap();

        image_select
            .value()
            .attr("src")
            .unwrap()
            .to_string()
            .replace("../", "")
    };
    let url = Url::parse(&format!("http://example.com/{}", image_path)).unwrap();
    let image_url = url
        .query_pairs()
        .find(|(key, _)| key == "id")
        .unwrap().1;
    let image = image_url.to_string();

    let base64 = fetch_image(&token.host, &token.cookie, &image).await;

    format!("{}{}", BASE64_IMAGE_HEAD, base64)
}

async fn get_datas(document: &str) -> Vec<UserDataValues> {
    let doc = Html::parse_document(document);
    let objs = doc.select(&DATA_SELECTOR).collect::<Vec<_>>();
    let mut vector: Vec<UserDataValues> = Vec::new();

    for ele in objs {
        let mut index: usize = 1;
        let mut felids: Vec<_> = ele.select(&FELIDS_SELECTOR).collect();

        if felids.len() > 4 {
            felids.remove(0);
        }

        for _ in &felids {
            if index % 2 == 0 && index != 0 {
                vector.push(UserDataValues {
                    name: felids[index - 2]
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .replace(" ", "")
                        .replace("　", "")
                        .replace("\n", ""),
                    value: felids[index - 1]
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .replace(" ", "")
                        .replace("\r\n", "")
                        .replace("\n", "")
                })
            }

            index += 1;
        }
    }

    vector
}

#[get("/getUserInfo")]
pub async fn api(auth: AuthorizationToken<AuthToken>) -> APIResponseJSON<UserData> {
    let token = auth.0;

    let page = combine_page_path(&token.host, APIPaths::Profile);

    let data = match http_get(&page, Some(create_auth_header(&token.cookie))).await {
        Ok(d) => d,
        Err(err) => return Err(generate_http_error(API_PATH, err))
    };

    let is_success = data.status().is_success();
    let html_doc = data.text().await.unwrap();

    if !is_success && !is_document_logined(&Html::parse_document(&html_doc)) {
        return Err(generate_session_expire_error(API_PATH))
    }

    let (image_data, profile_data) = join!(get_image(token, &html_doc), get_datas(&html_doc));

    Ok(Custom(Status::Ok, Json(UserData {
        message: "Get user profile successful".to_owned(),
        data: UserCollect {
            data: profile_data,
            profileImg: image_data
        }
    })))
}