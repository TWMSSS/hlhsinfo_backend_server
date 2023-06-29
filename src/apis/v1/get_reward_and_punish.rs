use lazy_static::lazy_static;
use rocket::{response::status::Custom, http::Status, serde::json::Json};
use scraper::{Selector, Html};
use tokio::join;

use crate::{
    request_handler::AuthorizationToken,
    types::{AuthToken, APIResponseJSON, RewardAndPunishData, RewardAndPunishStatus, RewardAndPunishDetailValue, RewardAndPunishCollect},
    http::APIPaths,
    utils::{combine_page_path, create_auth_header, generate_session_expire_error, convert_string_to_u32, http_get_err_handle, html_to_text}
};

lazy_static! {
    static ref SUMMARIZE_TABLE: Selector = Selector::parse("table > tbody").unwrap();
    static ref TR_SELECT: Selector = Selector::parse("tr").unwrap();
    static ref TD_SELECT: Selector = Selector::parse("td").unwrap();
    static ref ROW_DATA: Selector = Selector::parse("tr.dataRow").unwrap();
}

const API_PATH: &str = "/v1/getRewAndPun";

async fn get_summarize(html: &str) -> Vec<RewardAndPunishStatus> {
    let html = Html::parse_document(&html);
    let mut data: Vec<RewardAndPunishStatus> = Vec::new();

    let select = html.select(&SUMMARIZE_TABLE).collect::<Vec<_>>();
    let mut select = select
        .get(select.len() - 2)
        .unwrap()
        .select(&TR_SELECT)
        .collect::<Vec<_>>();

    select.remove(0);

    for ele in select {
        let mut tds = ele.select(&TD_SELECT).collect::<Vec<_>>();
        tds.remove(0);

        for index in (1..tds.len()).step_by(2) {
            data.push(RewardAndPunishStatus {
                r#type: html_to_text(tds[index - 1]),
                times: convert_string_to_u32(&html_to_text(tds[index])) as u16
            });
        }
    }

    data
}

async fn get_detail(html: &str) -> Vec<RewardAndPunishDetailValue> {
    let html = Html::parse_document(html);
    let mut data: Vec<RewardAndPunishDetailValue> = Vec::new();
    let select = html.select(&ROW_DATA).collect::<Vec<_>>();

    for ele in select {
        let tds = ele.select(&TD_SELECT)
            .map(|s| html_to_text(s))
            .collect::<Vec<_>>();

        data.push(RewardAndPunishDetailValue {
            r#type: tds[0].clone(),
            start: tds[1].clone(),
            signed: tds[2].clone(),
            reason: tds[3].clone(),
            execute: tds[4].clone(),
            sold: if tds[5].eq(&char::from_u32(0xa0).unwrap().to_string()) { None } else { Some(tds[5].clone()) },
            year: convert_string_to_u32(&tds[6]) as u16
        });
    }

    data
}

#[get("/getRewAndPun")]
pub async fn api(auth: AuthorizationToken<AuthToken>) -> APIResponseJSON<RewardAndPunishData> {
    let token = auth.0;

    let page = combine_page_path(&token.host, APIPaths::RewardAndPunish);

    let data = http_get_err_handle(API_PATH, &page, Some(create_auth_header(&token.cookie))).await?;
    if !data.status().is_success() {
        return Err(generate_session_expire_error(API_PATH))
    }

    let raw = data.text().await.unwrap();
    let (summarize, detail) = join!(get_summarize(&raw), get_detail(&raw));

    Ok(Custom(Status::Ok, Json(RewardAndPunishData {
        message: "Get reward and punish successful".to_owned(),
        data: RewardAndPunishCollect {
            status: summarize,
            detail
        }
    })))
}