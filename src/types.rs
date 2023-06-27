use rocket::{response::status::Custom, serde::json::Json};
use serde::{Deserialize, Serialize};

pub type ErrorReturn = Custom<Json<ErrorResponse>>;
pub type HTTPResponse<T> = Result<T, ErrorReturn>;
pub type APIResponse<T> = HTTPResponse<Custom<T>>;
pub type APIResponseJSON<T> = APIResponse<Json<T>>;

// Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub provider: String,
    pub share_expired: u32,

    pub logininfo_expired: u16,
    pub login_status_expired: u16,

    pub failed_expried: u32,
    pub failed_times_lock: u8,

    pub cache_enabled: bool,
    pub cache_expired: u16,
    
    pub check_cycle: u16,
    pub enable_record: bool
}

pub struct CacheKeyData {
    pub id: Vec<u8>,
    pub key: Vec<u8>,
    pub iv: Vec<u8>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInfoAuthToken {
    pub host: String,
    pub site_key: String,
    pub cookie: String,
    pub need_captcha: bool,

    // JWT config
    pub iat: u128,       // issued at
    pub exp: u128        // expired at
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    pub host: String,
    pub cookie: String,
    pub user_data: UserProfileShortValue,

    // JWT config
    pub iat: u128,       // issued at
    pub exp: u128        // expired at
}

// Alive JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct Alive {
    pub message: String,
    pub timestamp: u128,
    pub provider: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseErrorAt {
    pub api: Option<String>,
    pub trace: Option<String>,
    pub at: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub timestamp: u128,
    pub wrong: Option<ResponseErrorAt>
}

// API: /getLoginInfo
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LoginInfo {
    pub authToken: String,
    pub need_captcha: bool
}

// API: /login
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub message: String,
    pub authtoken: String
}

// API: /getUserInfo
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataValues {
    pub name: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserCollect {
    pub data: Vec<UserDataValues>,
    pub profileImg: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub message: String,
    pub data: UserCollect
}

// API: /getUserInfoShort
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct UserProfileShortValue {
    pub className: String,
    pub classNumber: String,
    pub gender: String,
    pub schoolNumber: String,
    pub userName: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileShortData {
    pub message: String,
    pub data: UserProfileShortValue
}

// API: /getAvailableScore
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AvailableScoreValue {
    pub name: String,
    pub term: u8,
    pub testID: String,
    pub times: u8,
    pub r#type: u8,
    pub year: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableScoreData {
    pub message: String,
    pub data: Vec<AvailableScoreValue>
}

// API: /getScoreInfo
#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreDataValue {
    pub name: String,
    pub score: u8,
    pub gpa: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreExtraData {
    pub r#type: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreUnpass {
    pub name: String,
    pub r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreDataCollect {
    pub data: Vec<ScoreDataValue>,
    pub extra: Vec<ScoreExtraData>,
    pub unpass: Vec<ScoreUnpass>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreData {
    pub message: String,
    pub data: ScoreDataCollect
}

// API: /getRewAndPun
#[derive(Debug, Serialize, Deserialize)]
pub struct RewardAndPunishDetailValue {
    pub execute: String,
    pub reason: String,
    pub signed: String,
    pub sold: Option<String>,
    pub start: String,
    pub r#type: String,
    pub year: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardAndPunishStatus {
    pub r#type: String,
    pub times: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardAndPunishCollect {
    pub detail: Vec<RewardAndPunishDetailValue>,
    pub status: Vec<RewardAndPunishStatus>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardAndPunishData {
    pub message: String,
    pub data: RewardAndPunishCollect
}

// API: /getLack
#[derive(Debug, Serialize, Deserialize)]
pub struct LackRecordValue {
    pub data: Option<Vec<String>>,
    pub date: String,
    pub week: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LackStatusValue {
    pub name: String,
    pub value: u16
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LackStatus {
    pub termDown: Vec<LackRecordValue>,
    pub termUp: Vec<LackRecordValue>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LackCollect {
    pub record: Vec<LackRecordValue>,
    pub total: LackStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LackData {
    pub message: String,
    pub data: LackCollect
}

// API: /getAllScores
#[derive(Debug, Serialize, Deserialize)]
pub struct AllScoreNormalDataValue {
    pub name: String,
    pub value: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllScoreNormalData {
    pub name: String,
    pub values: Vec<AllScoreNormalDataValue>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllScoreTestDataInfo {
    pub name: String,
    pub term: u8,
    pub test: u8,
    pub year: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllScoreTestDataValue {
    pub name: AllScoreTestDataInfo,
    pub value: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllScoreTestData {
    pub name: String,
    pub values: Vec<AllScoreTestDataValue>
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AllScoreTestCollect {
    pub dataNormal: Vec<AllScoreNormalData>,
    pub dataTest: Vec<AllScoreTestData>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllScoreData {
    pub message: String,
    pub data: AllScoreTestCollect
}

// API: /getScheduleList
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleListValues {
    pub name: String,
    pub class: String,
    pub teacher: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleListCollect {
    pub schedules: Vec<ScheduleListValues>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleListData {
    pub message: String,
    pub data: ScheduleListCollect
}

// API: /getSchedule
// TODO:
// Compete this api
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Schedule

// API: /shareScore
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ShareScoreCollect {
    pub id: String,
    pub createdTimestamp: u128,
    pub expiredTimestamp: u128
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareScoreData {
    pub message: String,
    pub data: ShareScoreCollect
}

// API: /getShared
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSharedScoreInfo {
    pub term: u8,
    pub times: u8,
    pub year: u8
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GetSharedCollect {
    pub data: Vec<ScoreDataValue>,
    pub extra: Vec<ScoreExtraData>,
    pub unpass: Vec<ScoreUnpass>,
    pub scoreInfo: GetSharedScoreInfo,
    pub sharedID: String,
    pub userInfo: UserProfileShortValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSharedData {
    pub message: String,
    pub data: GetSharedCollect
}