use rocket::{request::{FromRequest, Outcome}, http::Status, serde::json::Json, form::{Form, FromForm}, data::{FromData, self}};
use serde::de::DeserializeOwned;

use crate::secure::{decode_jwt, JWTError};

pub enum AuthorizationType {
    LoginAuthToken,
    AuthToken
}

#[derive(Debug, Clone)]
pub struct AuthorizationToken<T>(pub T);

#[derive(Debug)]
pub enum AuthTokenError {
    MissingToken,
    TokenExpired
}

#[async_trait]
impl<'r, T> FromRequest<'r> for AuthorizationToken<T>
where
    T: DeserializeOwned
{
    type Error = AuthTokenError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        if let Some(auth_header) = headers.get_one("Authorization") {
            let auth_parts: Vec<&str> = auth_header.split_whitespace().collect();
            if auth_parts.len() == 2 && auth_parts[0] == "Bearer" {
                let auth = auth_parts[1].to_string();
                
                match decode_jwt::<T>(&auth) {
                    Ok(tkn) => return Outcome::Success(AuthorizationToken(tkn.claims)),
                    Err(err) => match err {
                        JWTError::Expired => return Outcome::Failure((Status::Forbidden, AuthTokenError::TokenExpired)),
                        JWTError::Invalid => return Outcome::Failure((Status::BadGateway, AuthTokenError::MissingToken))
                    }
                }
            }
        }
        Outcome::Failure((Status::BadGateway, AuthTokenError::MissingToken))
    }
}

pub enum IncomingDataWrapper<T> {
    Form(Form<T>),
    Json(Json<T>)
}

#[derive(Debug)]
pub enum IncomingError {
    MissingArguments(Vec<String>),
    InvalidMediaType
}

#[async_trait]
impl<'r, T: 'r + Send> FromData<'r> for IncomingDataWrapper<T>
where
    T: DeserializeOwned + FromForm<'r>
{
    type Error = IncomingError;

    async fn from_data(req: &'r rocket::Request<'_>, data: rocket::Data<'r>) -> data::Outcome<'r, Self, Self::Error> {
        match req.content_type() {
            Some(media) => {
                if media.is_form_data() {
                    return Form::<T>::from_data(req, data).await
                        .and_then(|f| data::Outcome::Success(IncomingDataWrapper::<T>::Form(f)))
                        .failure_then(|err| {
                            let err = err.1;
                            let mut argument_missing: Vec<String> = Vec::new();
                            for e in err {
                                argument_missing.push(e.name.unwrap().to_string());
                            }
                            data::Outcome::Failure((Status::BadRequest, IncomingError::MissingArguments(argument_missing)))
                        })
                }
                
                if media.is_json() {
                    return Json::<T>::from_data(req, data).await
                        .and_then(|f| data::Outcome::Success(IncomingDataWrapper::<T>::Json(f)))
                        .failure_then(|err| {
                            let err = err.1;
                            data::Outcome::Failure((Status::BadRequest, IncomingError::MissingArguments(vec![err.to_string()])))
                        })
                }

                data::Outcome::Failure((Status::BadRequest, IncomingError::InvalidMediaType))
            },
            _ => data::Outcome::Failure((Status::BadRequest, IncomingError::InvalidMediaType))
        }
    }
}

pub fn decode_incoming<T>(incoming: IncomingDataWrapper<T>) -> T {
    match incoming {
        IncomingDataWrapper::Form(d) => d.into_inner(),
        IncomingDataWrapper::Json(d) => d.into_inner()
    }
}