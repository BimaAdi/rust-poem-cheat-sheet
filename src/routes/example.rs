use crate::schema::{
    common::InternalServerErrorResponse,
    example::{
        BadRequestResponse, ExampleFormRequest, ExampleFormResponse, ExampleJSON,
        ExampleMultipleResponse, ExamplePathQueryResponse, OkExampleResponse,
        UnprocesableEntityResponse,
    },
};
use poem::Request;
use poem_openapi::{
    auth::ApiKey,
    param::{Path, Query},
    payload::{Json, PlainText},
    OpenApi, SecurityScheme, Tags,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserApiKey {
    pub token: String,
}

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "header",
    checker = "api_checker"
)]
pub struct MyApiKeyAuthorization(UserApiKey);

pub async fn api_checker(_req: &Request, api_key: ApiKey) -> Option<UserApiKey> {
    Some(UserApiKey { token: api_key.key })
}

#[derive(Tags)]
enum ApiExampleTags {
    /// Example various poem implementation
    Example,
}

pub struct ApiExample;

#[OpenApi]
impl ApiExample {
    #[oai(
        path = "/example/hello",
        method = "get",
        tag = "ApiExampleTags::Example"
    )]
    async fn hello(&self) -> PlainText<String> {
        tracing::info!("GET /example/hello");
        PlainText("hello".to_string())
    }

    #[oai(
        path = "/example/path-query/:path",
        method = "get",
        tag = "ApiExampleTags::Example"
    )]
    async fn path_query(
        &self,
        path: Path<String>,
        query_1: Query<Option<String>>,
        query_2: Query<Option<i64>>,
    ) -> Json<ExamplePathQueryResponse> {
        // build log
        let mut url = format!("GET /example/path-query/{}", path.0);
        if query_1.0.is_some() || query_2.0.is_some() {
            url.push('?');
        }
        if query_1.0.is_some() {
            url.push_str(format!("query_1={}", query_1.0.clone().unwrap()).as_str());
        }
        if query_2.0.is_some() {
            url.push_str(format!("query_2={}", query_2.0.unwrap()).as_str());
        }
        tracing::info!("{}", url);

        Json(ExamplePathQueryResponse {
            path: path.0,
            query_1: query_1.0,
            query_2: query_2.0,
        })
    }

    #[oai(
        path = "/example/multiple-response",
        method = "get",
        tag = "ApiExampleTags::Example"
    )]
    async fn multiple_response(&self, status: Query<i32>) -> ExampleMultipleResponse {
        match status.0 {
            200 => {
                tracing::info!("Ok");
                ExampleMultipleResponse::Ok(Json(OkExampleResponse {
                    data: "some data".to_string(),
                }))
            }
            400 => {
                tracing::info!("bad request");
                ExampleMultipleResponse::BadRequest(Json(BadRequestResponse {
                    validation_error: "some validataion error".to_string(),
                }))
            }
            500 => {
                tracing::error!("something wrong");
                ExampleMultipleResponse::InternalServerError(Json(InternalServerErrorResponse {
                    error: "some error".to_string(),
                }))
            }
            _ => {
                tracing::warn!("invalid status");
                ExampleMultipleResponse::Unprocessable(Json(UnprocesableEntityResponse {
                    validation_error: format!("invalid status = {}", status.0),
                }))
            }
        }
    }

    #[oai(
        path = "/example/json",
        method = "post",
        tag = "ApiExampleTags::Example"
    )]
    async fn json_payload_and_response(&self, json: Json<ExampleJSON>) -> Json<ExampleJSON> {
        Json(ExampleJSON {
            key1: json.key1.to_string(),
            key2: json.key2,
            key3: json.key3,
        })
    }

    #[oai(
        path = "/example/form",
        method = "post",
        tag = "ApiExampleTags::Example"
    )]
    async fn form_payload(&self, form: ExampleFormRequest) -> Json<ExampleFormResponse> {
        println!("{:?}", form);
        let file = form.file.map(|x| x.file_name().unwrap_or("").to_string());
        let files = form
            .files
            .iter()
            .map(|x| x.file_name().unwrap_or("").to_string())
            .collect();
        Json(ExampleFormResponse {
            key1: form.key1,
            key2: form.key2,
            key3: form.key3,
            key4: form.key4,
            key5: form.key5,
            file,
            files,
        })
    }

    #[oai(
        path = "/example/auth",
        method = "get",
        tag = "ApiExampleTags::Example"
    )]
    async fn auth_example(&self, auth: MyApiKeyAuthorization) -> PlainText<String> {
        PlainText(auth.0.token.to_string())
    }
}
