use poem_openapi::{
    types::{ParseFromJSON, ToJSON},
    Object,
};

#[derive(Object)]
pub struct PaginateResponse<T: ToJSON + ParseFromJSON> {
    pub page: i32,
    pub page_size: i32,
    pub num_data: i32,
    pub num_page: i32,
    pub results: Vec<T>,
}

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
