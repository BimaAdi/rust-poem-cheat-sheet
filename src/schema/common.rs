use poem_openapi::Object;

#[derive(Object)]
pub struct NotFoundResponse {
    pub message: String,
}

#[derive(Object)]
pub struct InternalServerErrorResponse {
    pub error: String,
}

impl InternalServerErrorResponse {
    pub fn new(filepath: &str, function: &str, identifier: &str, err: &str) -> Self {
        Self {
            error: format!(
                "error: on {}::{} iden:{} error:{}",
                filepath, function, identifier, err
            )
            .to_string(),
        }
    }
}
