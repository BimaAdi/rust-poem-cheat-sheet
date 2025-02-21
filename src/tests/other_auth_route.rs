use std::sync::Arc;

use poem::test::TestClient;
use sqlx::SqlitePool;

use crate::{init_openapi_routes, AppState};

#[tokio::test]
async fn auth_example_same_auth() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/other-auth-route/same")
        .header("X-API-Key", "boo")
        .send()
        .await;

    // Expect
    resp.assert_text("boo").await;
}

#[tokio::test]
async fn auth_example_different_auth() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/other-auth-route/different")
        .header("X-API-Key", "bar")
        .send()
        .await;

    // Expect
    resp.assert_text("bar").await;
}
