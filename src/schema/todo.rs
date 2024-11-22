use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};

use super::common::{InternalServerErrorResponse, NotFoundResponse, OkResponse, PaginateResponse};

#[derive(Object)]
pub struct TodoPaginateDetail {
    pub id: i32,
    pub todo: String,
    pub is_done: bool,
}

#[derive(ApiResponse)]
pub enum TodoPaginateResponses {
    #[oai(status = 200)]
    Ok(Json<PaginateResponse<TodoPaginateDetail>>),

    #[oai(status = 500)]
    InternalServerError(Json<InternalServerErrorResponse>),
}

#[derive(Object)]
pub struct TodoDetailFound {
    pub id: i32,
    pub todo: String,
    pub is_done: bool,
}

#[derive(ApiResponse)]
pub enum TodoDetailResponses {
    #[oai(status = 200)]
    Ok(Json<TodoDetailFound>),

    #[oai(status = 404)]
    NotFound(Json<NotFoundResponse>),

    #[oai(status = 500)]
    InternalServerError(Json<InternalServerErrorResponse>),
}

#[derive(Object, Deserialize)]
pub struct TodoCreateRequest {
    pub todo: String,
    pub is_done: bool,
}

#[derive(Object, Serialize)]
pub struct TodoCreateOk {
    pub id: i32,
    pub todo: String,
    pub is_done: bool,
}

#[derive(ApiResponse)]
pub enum TodoCreateResponses {
    #[oai(status = 200)]
    Ok(Json<TodoCreateOk>),
}

#[derive(ApiResponse)]
pub enum TodoDeleteResponses {
    #[oai(status = 200)]
    Ok(Json<OkResponse>),

    #[oai(status = 404)]
    NotFound(Json<NotFoundResponse>),

    #[oai(status = 500)]
    InternalServerError(Json<InternalServerErrorResponse>),
}
