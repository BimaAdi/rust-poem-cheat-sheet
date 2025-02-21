use poem_openapi::{payload::PlainText, OpenApi, Tags};

use crate::security::{MyApiKeyAuthorization, OtherAuthorization};

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
