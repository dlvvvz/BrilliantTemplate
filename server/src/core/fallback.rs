use actix_web::{error::{JsonPayloadError, UrlencodedError, QueryPayloadError, PathError}, web, HttpRequest, Responder};
use crate::core::response::{code, Response};

// Not found
pub async fn default() -> impl Responder {
    Response::new(code::Common::NotFound)
}

// Json error
pub fn json_error(_err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::InternalError::from_response(
        "",
        Response::new(code::Validation::InvalidBody).build(),
    ).into()
}

// Form error
pub fn form_error(_err: UrlencodedError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::InternalError::from_response(
        "",
        Response::new(code::Validation::InvalidBody).build(),
    ).into()
}

// Query error
pub fn query_error(_err: QueryPayloadError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::InternalError::from_response(
        "",
        Response::new(code::Validation::InvalidFormat).build(),
    ).into()
}

// Path error
pub fn path_error(_err: PathError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::InternalError::from_response(
        "",
        Response::new(code::Validation::InvalidFormat).build(),
    ).into()
}

// Configure
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(json_error))
        .app_data(web::FormConfig::default().error_handler(form_error))
        .app_data(web::QueryConfig::default().error_handler(query_error))
        .app_data(web::PathConfig::default().error_handler(path_error))
        .default_service(web::route().to(default));
}