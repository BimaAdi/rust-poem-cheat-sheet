use poem::{http::StatusCode, test::TestClient};
use serde_json::json;
use serial_test::serial;
use sqlx::SqlitePool;

use crate::{init_openapi_routes, tests::utils::setup_database, AppState};

#[tokio::test]
#[serial]
async fn get_detail_todo() {
    // Given
    let db_url = "sqlite.db";
    setup_database(db_url).await;
    let pool = SqlitePool::connect(format!("sqlite:./{}?mode=rwc", &db_url).as_str())
        .await
        .unwrap();
    let app_state = AppState { db: pool.clone() };
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);
    let json_payload = json!({
        "todo": "first todo",
        "is_done": false
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;
    resp.assert_status_is_ok();

    // When 1
    let resp = cli
        .get(format!("/api/todo/{}", 1))
        .body_json(&json_payload)
        .send()
        .await;

    // Expect 1
    resp.assert_status_is_ok();
    resp.assert_json(json!({
        "id": 1,
        "todo": "first todo",
        "is_done": false
    }))
    .await;

    // When 2
    let resp = cli
        .get(format!("/api/todo/{}", 2))
        .body_json(&json_payload)
        .send()
        .await;

    // Expect 2
    resp.assert_status(StatusCode::NOT_FOUND);
    resp.assert_json(json!({
        "message": "todo with id 2 not found"
    }))
    .await;
}

#[tokio::test]
#[serial]
async fn create_todo() {
    // Given
    let db_url = "sqlite.db";
    setup_database(db_url).await;
    let pool = SqlitePool::connect(format!("sqlite:./{}?mode=rwc", &db_url).as_str())
        .await
        .unwrap();
    let app_state = AppState { db: pool.clone() };
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let json_payload = json!({
        "todo": "first todo",
        "is_done": false
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;

    // Expect
    resp.assert_status_is_ok();
    resp.assert_json(json!({
        "id": 1,
        "todo": "first todo",
        "is_done": false
    }))
    .await;
    let data: (i32, String, i32) =
        sqlx::query_as("SELECT id, todo, is_done FROM todo WHERE id = ?")
            .bind(1)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(data.0, 1);
    assert_eq!(data.1, "first todo".to_string());
    assert_eq!(data.2, 0);
}
