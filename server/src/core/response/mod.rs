pub mod data;
pub mod code;

use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;
use serde_json::{Map, Value, to_value};


#[derive(Serialize, Clone, Debug)]
pub struct Response {
    code: Value,
    data: Value,
    #[serde(skip_serializing)]
    status: StatusCode,
}


impl Response {
    pub fn new(
        code: impl code::Code
    ) -> Self {
        let parsed_code = to_value(&code).unwrap_or_else(|_| Value::String("err_Internal".to_string()));
        Self {
            code: parsed_code,
            data: Value::Object(Map::new()),
            status: code.status()
        }
    }
    pub fn data(mut self, data: impl data::DataTemplate) -> Self {
        self.data = to_value(data).unwrap_or_else(|_| Value::Object(Map::new()));
        self
    }
    pub fn build(self) -> HttpResponse {
        HttpResponse::build(self.status)
            .json(self)
    }
}

impl actix_web::Responder for Response {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.build()
    }
}