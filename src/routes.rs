use rocket::{figment::Figment, serde::json::Json, Rocket, Build};

use crate::{apis, types, utils, cors::CORS, config, error::HTTPError};

#[get("/")]
fn home() -> Json<types::Alive> {
    Json(types::Alive {
        message: String::from(utils::HELLO_MESSAGE),
        timestamp: utils::get_timestamp_millisec(),
        provider: config::read_config().provider
    })
}

#[catch(400)]
fn err_bad_request() -> Json<types::ErrorResponse> {
    Json(types::ErrorResponse { 
        message: String::from(HTTPError::BadRequest.message()),
        timestamp: utils::get_timestamp_millisec(),
        wrong: None
    })
}

#[catch(401)]
fn err_unauthorized() -> Json<types::ErrorResponse> {
    Json(types::ErrorResponse { 
        message: String::from(HTTPError::Unauthorized.message()),
        timestamp: utils::get_timestamp_millisec(),
        wrong: None
    })
}

#[catch(403)]
fn err_forbidden() -> Json<types::ErrorResponse> {
    Json(types::ErrorResponse { 
        message: String::from(HTTPError::Forbidden.message()),
        timestamp: utils::get_timestamp_millisec(),
        wrong: None
    })
}

#[catch(404)]
fn err_not_found(req: &rocket::Request) -> Json<types::ErrorResponse> {
    Json(types::ErrorResponse { 
        message: String::from(HTTPError::FileNotFound.message()),
        timestamp: utils::get_timestamp_millisec(),
        wrong: Some(types::ResponseErrorAt {
            api: Some(req.uri().to_string()),
            trace: Some(String::from("")),
            at: None
        })
    })
}

#[catch(500)]
fn err_server_error() -> Json<types::ErrorResponse> {
    Json(types::ErrorResponse { 
        message: String::from(HTTPError::ServerError.message()),
        timestamp: utils::get_timestamp_millisec(),
        wrong: None
    })
}

#[catch(502)]
fn err_bad_gateway() -> Json<types::ErrorResponse> {
    Json(types::ErrorResponse { 
        message: String::from(HTTPError::RemoteServiceUnavailable.message()),
        timestamp: utils::get_timestamp_millisec(),
        wrong: None
    })
}

fn server_init(config: Figment) -> Rocket<Build> {
    let finit = rocket::custom(config)
        .attach(CORS)
        .register("/", catchers![err_bad_request, err_unauthorized, err_forbidden, err_not_found, err_server_error, err_bad_gateway])
        .mount("/", routes![home])
        .mount("/v1", routes![home]);

    apis::init_api_routes(finit)
}

pub fn create_server(config: Figment) -> Rocket<Build> {
    server_init(config)
}