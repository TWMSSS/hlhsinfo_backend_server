use lazy_static::lazy_static;
use rocket::{response::status::Custom, serde::json::Json, http::Status};
use scraper::Selector;
use url::{Url, form_urlencoded::Parse};

use crate::{
    types::{APIResponseJSON, AvailableScoreData, AuthToken, AvailableScoreValue},
    request_handler::AuthorizationToken, http::{http_get_html, APIPaths}, utils::{combine_page_path, create_auth_header, generate_http_error, convert_string_to_u32, is_document_logined, generate_session_expire_error}
};

lazy_static! {
    static ref LIST_SELECTOR: Selector = Selector::parse("#ddlExamList > option").unwrap();
}

const API_PATH: &str = "/v1/getAvailableScore";

fn find_string(parse_url: &Parse<'_>, param: &str) -> String {
    let find = parse_url.clone().find(|(key, _)| key == param);
    
    match find {
        Some(data) => data.1.to_string(),
        None => "".to_owned()
    }
}

#[get("/getAvailableScore")]
pub async fn api(auth: AuthorizationToken<AuthToken>) -> APIResponseJSON<AvailableScoreData> {
    let token = auth.0;

    let page = combine_page_path(&token.host, APIPaths::ScoreList);

    let respond = match http_get_html(&page, Some(create_auth_header(&token.cookie))).await {
        Ok(data) => data,
        Err(err) => return Err(generate_http_error(API_PATH, err))
    };

    if !respond.code.is_success() && !is_document_logined(&respond.html) {
        return Err(generate_session_expire_error(API_PATH))
    }

    let mut selected = respond.html.select(&LIST_SELECTOR).collect::<Vec<_>>();
    let mut data: Vec<AvailableScoreValue> = Vec::new();

    selected.remove(0);
    selected.remove(0);

    for ele in selected {
        let parse_url_string = ele.value().attr("value").unwrap().to_string();
        let parse_url = Url::parse(&format!("http://example.com/{}", parse_url_string)).unwrap();
        let search_params = parse_url.query_pairs();

        println!("{}: {:?}", parse_url_string, parse_url);
        
        let inner = ele.inner_html();
        let test_id = find_string(&search_params, "number");

        data.push(AvailableScoreValue {
            name: inner.clone(),
            year: convert_string_to_u32(&find_string(&search_params, "thisyear")) as u8,
            term: convert_string_to_u32(&find_string(&search_params, "thisterm")) as u8,
            testID: test_id.clone(),
            times: convert_string_to_u32(&test_id[3..4]) as u8,
            r#type: if inner.contains("平時成績") { 2 } else { 1 }
        });
    }

    Ok(Custom(Status::Ok, Json(AvailableScoreData {
        message: "Get available score data successful".to_owned(),
        data
    })))
}