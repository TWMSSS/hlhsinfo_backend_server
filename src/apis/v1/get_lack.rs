use lazy_static::lazy_static;
use rocket::{response::status::Custom, http::Status, serde::json::Json};
use scraper::Selector;

use crate::{
    request_handler::AuthorizationToken,
    types::{AuthToken, APIResponseJSON, LackData, LackStatusValue, LackRecordValue, LackCollect, LackStatus},
    utils::{combine_page_path, http_get_html_err_handle, create_auth_header, html_to_text, convert_string_to_u32},
    http::APIPaths
};

lazy_static! {
    static ref SUMMARIZE_TABLE_SELECT: Selector = Selector::parse("table.si_12.collapse.padding2.spacing0").unwrap();
    static ref SUMMARIZE_RECORD_SELECT: Selector = Selector::parse("tr").unwrap();
    static ref TD_SELECT: Selector = Selector::parse("td").unwrap();
    static ref TABLE_SELECT: Selector = Selector::parse("table.padding2.spacing0").unwrap();
    static ref RECORD_SELECT: Selector = Selector::parse("tr:not(.td_03.si_12.le_05.top.center)").unwrap();
}

const API_PATH: &str = "/v1/getLack";

#[get("/getLack")]
pub async fn api(auth: AuthorizationToken<AuthToken>) -> APIResponseJSON<LackData> {
    let token = auth.0;

    let page = combine_page_path(&token.host, APIPaths::Lack);
    let data = http_get_html_err_handle(API_PATH, &page, Some(create_auth_header(&token.cookie))).await?;

    let summary = data.html
        .select(&SUMMARIZE_TABLE_SELECT)
        .next()
        .unwrap()
        .select(&SUMMARIZE_RECORD_SELECT)
        .filter(|e| e
            .select(&TD_SELECT)
            .next()
            .unwrap()
            .value()
            .attr("colspan")
            .is_none())
        .map(|ele| ele
            .select(&TD_SELECT)
            .map(|e| html_to_text(e))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut summary_term_up_list: Vec<LackStatusValue> = Vec::new();
    let mut summary_term_down_list: Vec<LackStatusValue> = Vec::new();

    for i in 0..18 {
        summary_term_up_list.push(LackStatusValue {
            name: summary[0][i].clone(),
            value: convert_string_to_u32(&summary[1][i]) as u16
        });

        summary_term_down_list.push(LackStatusValue {
            name: summary[2][i].clone(),
            value: convert_string_to_u32(&summary[3][i]) as u16
        });
    }

    let record = data.html
        .select(&TABLE_SELECT)
        .next()
        .unwrap()
        .select(&RECORD_SELECT)
        .map(|ele| {
            let mut i = 0;

            let mut datas: Vec<Option<String>> = Vec::new();
            let mut data: LackRecordValue = LackRecordValue { data: Vec::new(), date: String::new(), week: String::new() };
            let selt = ele.select(&TD_SELECT).collect::<Vec<_>>();
            for e in selt {
                i += 1;
                let text = html_to_text(e);
                match i {
                    1 => data.week = text,
                    2 => data.date = text,
                    _ => if i != 3 {
                        datas.push(if text != "" { Some(text) } else { None });
                    }
                }
            };

            data.data = datas;

            data
        })
        .collect::<Vec<_>>();

    Ok(Custom(Status::Ok, Json(LackData {
        message: "Get lack successful".to_owned(),
        data: LackCollect {
            record,
            total: LackStatus {
                termUp: summary_term_up_list,
                termDown: summary_term_down_list
            }
        }
    })))
}