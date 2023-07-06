use lazy_static::lazy_static;
use rocket::{response::status::Custom, serde::json::Json, http::Status};
use scraper::Selector;

use crate::{
    request_handler::AuthorizationToken,
    types::{AuthToken, APIResponseJSON, ScoreData, ErrorReturn, ScoreDataValue, ScoreUnpass, ScoreExtraData, ScoreDataCollect},
    utils::{self, combine_path, create_auth_header, http_get_html_err_handle, html_to_text, convert_string_to_u32, convert_string_to_f32},
    http::{APIPaths, ReplaceString}
};

lazy_static! {
    static ref TABLE_SELECT: Selector = Selector::parse("table[id=Table1] tr").unwrap();
    static ref TDS_SELECT: Selector = Selector::parse("td").unwrap();
    static ref SPAN_SELECT: Selector = Selector::parse("span").unwrap();
    static ref EXTRA_SELECT: Selector = Selector::parse("table.scoreTable-inline.padding0.spacing2.center tr td").unwrap();
}

const API_PATH: &str = "/v1/getScoreInfo";
fn error_message(code: Status, message: &str, at: Option<&str>) -> ErrorReturn {
    utils::error_message(API_PATH, code, message, at)
}

#[derive(Debug, FromForm)]
#[allow(non_snake_case)]
pub struct DataStruct {
    pub year: String,
    pub term: String,
    pub times: String,
    pub testID: String
}

#[get("/getScoreInfo?<params..>")]
#[allow(non_snake_case)]
pub async fn api(auth: AuthorizationToken<AuthToken>, params: Option<DataStruct>) -> APIResponseJSON<ScoreData> {
    let token = auth.0;
    let params = match params {
        Some(param) => param,
        None => return Err(error_message(Status::BadRequest, "Missing one or more arguments", Some("Arguments")))
    };

    let page = combine_path(&token.host, &APIPaths::Score.replace(vec![
        ReplaceString {
            match_string: "$year$".to_owned(),
            replacement: params.year
        },
        ReplaceString {
            match_string: "$term$".to_owned(),
            replacement: params.term
        },
        ReplaceString {
            match_string: "$testid$".to_owned(),
            replacement: params.testID
        }
    ]));

    let data = http_get_html_err_handle(API_PATH, &page, Some(create_auth_header(&token.cookie))).await?;

    if data.html.html().contains("尚未開放") {
        return Err(error_message(Status::NotFound, "Cannot find the score data", None))
    }

    let mut score_list: Vec<ScoreDataValue> = Vec::new();
    let mut unpass_list: Vec<ScoreUnpass> = Vec::new();

    let mut table_data = data.html
        .select(&TABLE_SELECT)
        .collect::<Vec<_>>();

    table_data.remove(0);

    for ele in table_data {
        let list = ele.select(&TDS_SELECT).collect::<Vec<_>>();

        let is_avaiable = match list.get(1) {
            Some(_) => true,
            None => false
        };
        
        if list.len() > 0 && is_avaiable {
            let score_name = html_to_text(list[0]).replace(" ", "");
            let element_score = list[1].select(&SPAN_SELECT).next().unwrap();
            let element_gpa = list[2].select(&SPAN_SELECT).next().unwrap();

            let score_numb = convert_string_to_u32(&html_to_text(element_score)
                .replace(" ", "")
                .replace("\r\n", "")
                .replace("\n", "")) as u8;
            let score_gpa = convert_string_to_f32(&html_to_text(element_gpa)
                .replace(" ", "")
                .replace("\r\n", "")
                .replace("\n", ""));

            score_list.push(ScoreDataValue {
                name: score_name.clone(),
                score: score_numb,
                gpa: score_gpa
            });

            if let Some(data) = element_score.value().attr("style") {
                if data.contains("red") {
                    unpass_list.push(ScoreUnpass {
                        r#type: "score".to_owned(),
                        name: score_name.clone()
                    });
                }
            }
            if element_gpa.value().has_class("unpass", scraper::CaseSensitivity::AsciiCaseInsensitive) {
                unpass_list.push(ScoreUnpass {
                    r#type: "gpa".to_owned(),
                    name: score_name.clone()
                });
            }
        }
    }

    let mut extra_list: Vec<ScoreExtraData> = Vec::new();
    let extra_info = data.html.select(&EXTRA_SELECT).collect::<Vec<_>>();

    for i in 0..extra_info.len() {
        if extra_info[i].value().has_class("score", scraper::CaseSensitivity::AsciiCaseInsensitive) {
            extra_list.push(ScoreExtraData {
                r#type: html_to_text(extra_info[i - 1]).replace("：", ""),
                value: html_to_text(extra_info[i]).replace(" ", "").replace("\r\n", "").replace("\n", "")
            });
        }
    }

    Ok(Custom(Status::Ok, Json(ScoreData {
        message: "Get score info successful".to_owned(),
        data: ScoreDataCollect {
            data: score_list,
            extra: extra_list,
            unpass: unpass_list
        }
    })))
}