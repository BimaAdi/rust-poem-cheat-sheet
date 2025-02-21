use std::sync::Arc;

use poem::{
    middleware::{AddData, AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use routes::{example::ApiExample, other_auth_route::ApiOtherAuth, todo::ApiTodo};
use sqlx::{Pool, Sqlite};

pub mod routes;
pub mod schema;
pub mod security;
mod tests;
pub mod utils;

pub struct AppState {
    pub db: Pool<Sqlite>,
}

pub fn init_openapi_routes(
    app_state: Arc<AppState>,
) -> CorsEndpoint<AddDataEndpoint<Route, Arc<AppState>>> {
    let openapi_route =
        OpenApiService::new((ApiExample, ApiTodo, ApiOtherAuth), "Poem Demo", "1.0").server("/api");
    let openapi_json_endpoint = openapi_route.spec_endpoint();
    let ui = openapi_route.swagger_ui();
    Route::new()
        .nest("/api", openapi_route)
        .nest("/docs", ui)
        .at("openapi.json", openapi_json_endpoint)
        .with(AddData::new(app_state))
        .with(Cors::new())
}
