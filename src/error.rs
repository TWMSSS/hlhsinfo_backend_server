pub enum HTTPError {
    BadRequest,
    Unauthorized,
    Forbidden,
    FileNotFound,
    ServerError,
    RemoteServiceUnavailable,

    AuthorizationTokenMissMatch,
    NotAValidHost,
    SessionExpired
}

impl HTTPError {
    pub fn message(&self) -> &'static str {
        match *self {
            HTTPError::BadRequest => "This request is incorrect. Please check your request.",
            HTTPError::Unauthorized => "You have to be authorized to access this api.",
            HTTPError::Forbidden => "You have no premission to access this api.",
            HTTPError::FileNotFound => "Cannot found api. Please check your api path.",
            HTTPError::ServerError => "Our server is unreachable this time.",
            HTTPError::RemoteServiceUnavailable => "Remote service is unavailable",

            HTTPError::AuthorizationTokenMissMatch => "This authorization token is not for this api",
            HTTPError::NotAValidHost => "This is not a valid host",
            HTTPError::SessionExpired => "This login session is expired, please login again"
        }
    }
}

pub enum FetchError {
    AuthError,
    FetchFailed
}