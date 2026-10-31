use actix_web::{error::{JsonPayloadError, UrlencodedError, QueryPayloadError, PathError}, web, HttpRequest, Responder};
use crate::core::response::{code, Response};

// Not found
pub async fn default() -> impl Responder {
    Response::new(code::Common::NotFound)
}

fn reject(resp: Response) -> actix_web::Error {
    actix_web::error::InternalError::from_response("", resp.build()).into()
}

// Json error
pub fn json_error(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    let code = match err {
        JsonPayloadError::OverflowKnownLength { .. } | JsonPayloadError::Overflow { .. } => {
            code::Validation::PayloadTooLarge
        }
        JsonPayloadError::ContentType => code::Validation::UnsupportedMediaType,
        JsonPayloadError::Deserialize(e) if e.is_data() => code::Validation::InvalidFormat,
        _ => code::Validation::InvalidBody,
    };
    reject(Response::new(code))
}

// Form error
pub fn form_error(err: UrlencodedError, _req: &HttpRequest) -> actix_web::Error {
    let code = match err {
        UrlencodedError::Overflow { .. } => code::Validation::PayloadTooLarge,
        UrlencodedError::ContentType => code::Validation::UnsupportedMediaType,
        _ => code::Validation::InvalidBody,
    };
    reject(Response::new(code))
}

// Query error
pub fn query_error(_err: QueryPayloadError, _req: &HttpRequest) -> actix_web::Error {
    reject(Response::new(code::Validation::InvalidFormat))
}

// Path error
pub fn path_error(_err: PathError, _req: &HttpRequest) -> actix_web::Error {
    reject(Response::new(code::Validation::InvalidFormat))
}

// Configure
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(json_error))
        .app_data(web::FormConfig::default().error_handler(form_error))
        .app_data(web::QueryConfig::default().error_handler(query_error))
        .app_data(web::PathConfig::default().error_handler(path_error))
        .default_service(web::route().to(default));
}