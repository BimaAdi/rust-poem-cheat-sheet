use poem::Request;
use poem_openapi::{auth::ApiKey, SecurityScheme};
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
pub struct MyApiKeyAuthorization(pub UserApiKey);

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
pub struct OtherAuthorization(pub OtherApiKey);

pub async fn api_checker_other(_req: &Request, api_key: ApiKey) -> Option<OtherApiKey> {
    Some(OtherApiKey { token: api_key.key })
}
