use poem_openapi::OpenApiService;
use routes::{example::ApiExample, todo::ApiTodo};

pub mod routes;
pub mod schema;

pub fn init_openapi_routes() -> OpenApiService<(ApiExample, ApiTodo), ()> {
    OpenApiService::new((ApiExample, ApiTodo), "Poem Demo", "1.0").server("/api")
}
