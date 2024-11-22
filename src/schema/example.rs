use poem_openapi::{payload::Json, types::multipart::Upload, ApiResponse, Multipart, Object};

use super::common::InternalServerErrorResponse;

#[derive(Object)]
pub struct ExamplePathQueryResponse {
    pub path: String,
    pub query_1: Option<String>,
    pub query_2: Option<i64>,
}

#[derive(Object)]
pub struct OkExampleResponse {
    pub data: String,
}

#[derive(Object)]
pub struct BadRequestResponse {
    pub validation_error: String,
}

#[derive(Object)]
pub struct UnprocesableEntityResponse {
    pub validation_error: String,
}

#[derive(ApiResponse)]
pub enum ExampleMultipleResponse {
    #[oai(status = 200)]
    Ok(Json<OkExampleResponse>),

    #[oai(status = 400)]
    BadRequest(Json<BadRequestResponse>),

    #[oai(status = 422)]
    Unprocessable(Json<UnprocesableEntityResponse>),

    #[oai(status = 500)]
    InternalServerError(Json<InternalServerErrorResponse>),
}

#[derive(Object)]
pub struct ExampleJSON {
    pub key1: String,
    pub key2: i32,
    pub key3: bool,
}

fn key_5_default() -> i32 {
    3
}

#[derive(Multipart, Debug)]
pub struct ExampleFormRequest {
    pub key1: String,
    pub key2: i32,
    pub key3: Option<String>,
    #[oai(validator(min_length = 1, max_length = 6))]
    pub key4: Option<String>,
    #[oai(rename = "key 5", default = "key_5_default")]
    pub key5: i32,
    pub file: Option<Upload>,
    pub files: Vec<Upload>,
}

#[derive(Object)]
pub struct ExampleFormResponse {
    pub key1: String,
    pub key2: i32,
    pub key3: Option<String>,
    pub key4: Option<String>,
    pub key5: i32,
    pub file: Option<String>,
    pub files: Vec<String>,
}
