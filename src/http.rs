use reqwest::{Client, Response, StatusCode, header::HeaderMap, Method, redirect::Policy};
use scraper::Html;
use serde::Serialize;
use serde_urlencoded;

pub struct ReplaceString {
    pub match_string: String,
    pub replacement: String
}

pub enum APIPaths {
    Home,
    Login,
    LoginCaptcha,
    ScoreList,
    Score,
    Profile,
    ProfileImage,
    ProfileShort,
    ClassData,
    RewardAndPunish,
    Lack,
    AllScores,
    Schedule,
    ScheduleList
}

impl APIPaths {
    pub fn path(&self) -> &'static str {
        match *self {
            // Default page
            APIPaths::Home => "/",
            APIPaths::Login => "/login.asp",
            APIPaths::LoginCaptcha => "/image/vcode.asp",

            // Score information
            APIPaths::Score => "/selection_student/student_subjects_number.asp?action=%A6U%A6%A1%A6%A8%C1Z&thisyear=$year$&thisterm=$term$&number=$testid$&exam_name=hlhs",
            APIPaths::ScoreList => "/selection_student/student_subjects_number.asp?action=open_window_frame",
            APIPaths::AllScores => "/selection_student/grade_chart_all.asp",

            // Personal information
            APIPaths::Profile => "/selection_student/fundamental.asp",
            APIPaths::ProfileImage => "/utility/file1.asp?q=x&id=$imgid$",
            APIPaths::ProfileShort => "/student/selection_look_over_data.asp?look_over=right_below&school_class=",
            APIPaths::ClassData => "/student/selection_look_over_data.asp?look_over=right_top&school_class=&division=",

            // School schedule
            APIPaths::Schedule => "/student/school_class_tabletime.asp?teacher_classnumber=$class$&teacher_name=$teacher$",
            APIPaths::ScheduleList => "/student/select_preceptor.asp?action=open_sel",

            // Extra information
            APIPaths::Lack => "/selection_student/absentation_skip_school.asp",
            APIPaths::RewardAndPunish => "/selection_student/moralculture_%20bonuspenalty.asp",
        }
    }

    pub fn replace(&self, replacement: Vec<ReplaceString>) -> String {
        let mut origin = self.path().to_owned();

        for rp in &replacement {
            origin = origin.replace(&rp.match_string, &rp.replacement);
        }

        origin
    }
}

#[derive(Debug)]
pub enum HTTPErrorReturn {
    RequestError(reqwest::Error),
    StatusCodeError(StatusCode)
}

pub struct HeaderSetting {
    pub header: String,
    pub value: String
}

async fn http_request<T>(method: Method, url: &str, headers: Option<HeaderMap>, body: Option<T>) -> Result<Response, reqwest::Error>
where
    T: Serialize
{
    let redirect = Policy::none();

    let request = Client::builder()
        .redirect(redirect)
        .build()?;
    let mut request_builder = request
        .request(method, url);

    if let Some(header) = headers {
        request_builder = request_builder.headers(header);
    }

    if let Some(bd) = body {
        let body = serde_urlencoded::to_string(&bd).unwrap();
        request_builder = request_builder
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body);
    }

    let respond = request_builder.send();

    Ok(respond.await?)
}

#[derive(Debug, Clone)]
pub struct HTMLRespond {
    pub html: Html,
    pub code: StatusCode,
    pub header: HeaderMap
}

pub async fn http_get(url: &str, headers: Option<HeaderMap>) -> Result<Response, HTTPErrorReturn> {
    let respond = http_request::<()>(Method::GET, url, headers, None).await.map_err(HTTPErrorReturn::RequestError)?;

    let code = respond.status();

    if !code.is_success() && !code.is_redirection() {
        return Err(HTTPErrorReturn::StatusCodeError(code));
    }

    Ok(respond)
}

pub async fn http_get_html(url: &str, headers: Option<HeaderMap>) -> Result<HTMLRespond, HTTPErrorReturn> {
    let respond = http_get(url, headers).await?;

    let header = respond.headers().clone();
    let code = respond.status().clone();
    let respond_text = respond.text().await.unwrap();
    let doc = Html::parse_document(&respond_text);

    Ok(HTMLRespond { html: doc, code, header })
}

pub async fn http_post<T>(url: &str, body: T, headers: Option<HeaderMap>) -> Result<Response, HTTPErrorReturn>
where
    T: Serialize
{
    let respond = http_request::<T>(Method::POST, url, headers, Some(body)).await.map_err(HTTPErrorReturn::RequestError)?;

    let code = respond.status();

    if !code.is_success() && !code.is_redirection() {
        return Err(HTTPErrorReturn::StatusCodeError(code));
    }

    Ok(respond)
}