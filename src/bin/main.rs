use poem::{listener::TcpListener, Route};
use try_poem::init_openapi_routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = init_openapi_routes();
    let openapi_json_endpoint = api_service.spec_endpoint();
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .at("openapi.json", openapi_json_endpoint);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
