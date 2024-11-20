use poem::{
    middleware::{AddData, AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use routes::{example::ApiExample, todo::ApiTodo};
use sqlx::{Pool, Sqlite};

pub mod routes;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
}

pub fn init_openapi_routes(app_state: AppState) -> CorsEndpoint<AddDataEndpoint<Route, AppState>> {
    let openapi_route =
        OpenApiService::new((ApiExample, ApiTodo), "Poem Demo", "1.0").server("/api");
    let openapi_json_endpoint = openapi_route.spec_endpoint();
    let ui = openapi_route.swagger_ui();
    Route::new()
        .nest("/api", openapi_route)
        .nest("/docs", ui)
        .at("openapi.json", openapi_json_endpoint)
        .with(AddData::new(app_state.clone()))
        .with(Cors::new())
}
