use poem::Request;
use poem_openapi::{auth::ApiKey, payload::PlainText, OpenApi, SecurityScheme, Tags};
use serde::{Deserialize, Serialize};

// But there is bug in poem openapi, you cannot put on different file see: https://github.com/poem-web/poem/issues/915
// Because the field will become private, the work around that I found is by copy entire struct and named it with same name,
// since poem open api define securitySchemes name using struct name

// Same auth with route example
#[derive(Debug, Serialize, Deserialize)]
pub struct UserApiKey {
    pub token: String,
}

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

// Different auth with route example
#[derive(Debug, Serialize, Deserialize)]
pub struct OtherApiKey {
    pub token: String,
}

#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "header",
    checker = "api_checker_other"
)]
pub struct OtherAuthorization(OtherApiKey);

pub async fn api_checker_other(_req: &Request, api_key: ApiKey) -> Option<OtherApiKey> {
    Some(OtherApiKey { token: api_key.key })
}

#[derive(Tags)]
enum ApiOtherAuthTags {
    // Auth on diffrent route
    OtherAuth,
}

pub struct ApiOtherAuth;

#[OpenApi]
impl ApiOtherAuth {
    #[oai(
        path = "/other-auth-route/same",
        method = "get",
        tag = "ApiOtherAuthTags::OtherAuth"
    )]
    async fn auth_example_same_auth(&self, auth: MyApiKeyAuthorization) -> PlainText<String> {
        PlainText(auth.0.token.to_string())
    }

    #[oai(
        path = "/other-auth-route/different",
        method = "get",
        tag = "ApiOtherAuthTags::OtherAuth"
    )]
    async fn auth_example_different_auth(&self, auth: OtherAuthorization) -> PlainText<String> {
        PlainText(auth.0.token.to_string())
    }
}
