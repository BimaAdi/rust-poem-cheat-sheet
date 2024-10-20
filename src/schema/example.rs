use poem_openapi::{payload::Json, ApiResponse, Object};

#[derive(Object)]
pub struct ExamplePathQueryResponse {
    pub path: String,
    pub query_1: Option<String>,
    pub query_2: Option<i64>,
}

#[derive(Object)]
pub struct OkResponse {
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

#[derive(Object)]
pub struct InternalServerErrorResponse {
    pub error: String,
}

#[derive(ApiResponse)]
pub enum ExampleMultipleResponse {
    #[oai(status = 200)]
    Ok(Json<OkResponse>),

    #[oai(status = 400)]
    BadRequest(Json<BadRequestResponse>),

    #[oai(status = 422)]
    Unprocessable(Json<UnprocesableEntityResponse>),

    #[oai(status = 500)]
    InternalServerError(Json<InternalServerErrorResponse>),
}
