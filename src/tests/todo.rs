use std::sync::Arc;

use poem::{http::StatusCode, test::TestClient};
use serde_json::json;
use sqlx::SqlitePool;

use crate::{init_openapi_routes, AppState};

#[sqlx::test]
async fn get_paginate_todo(pool: SqlitePool) -> sqlx::Result<()> {
    // Given
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);
    let json_payload = json!({
        "todo": "first todo",
        "is_done": false
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;
    resp.assert_status_is_ok();
    let json_payload = json!({
        "todo": "second todo",
        "is_done": true
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;
    resp.assert_status_is_ok();

    // When 1
    let resp = cli.get("/api/todo").send().await;
    // println!("{:?}", resp.0.into_body().into_string().await);

    // Expect 1
    resp.assert_status(StatusCode::OK);
    resp.assert_json(json!({
        "page": 1,
        "page_size": 5,
        "num_data": 2,
        "num_page": 1,
        "results": vec![json!({
            "id": 2,
            "todo": "second todo",
            "is_done": true
        }), json!({
            "id": 1,
            "todo": "first todo",
        "is_done": false
        })]
    }))
    .await;

    // When 2
    let resp = cli
        .get("/api/todo")
        .query("page", &2)
        .query("page_size", &1)
        .send()
        .await;
    // println!("{:?}", resp.0.into_body().into_string().await);

    // Expect 2
    resp.assert_status(StatusCode::OK);
    resp.assert_json(json!({
        "page": 2,
        "page_size": 1,
        "num_data": 2,
        "num_page": 2,
        "results": vec![json!({
            "id": 1,
            "todo": "first todo",
        "is_done": false
        })]
    }))
    .await;
    Ok(())
}

#[sqlx::test]
async fn get_detail_todo(pool: SqlitePool) -> sqlx::Result<()> {
    // Given
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);
    let json_payload = json!({
        "todo": "first todo",
        "is_done": false
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;
    resp.assert_status_is_ok();

    // When 1
    let resp = cli.get(format!("/api/todo/{}", 1)).send().await;

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
    Ok(())
}

#[sqlx::test]
async fn create_todo(pool: SqlitePool) -> sqlx::Result<()> {
    // Given
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state.clone());
    let cli = TestClient::new(app);

    // When
    let json_payload = json!({
        "todo": "first todo",
        "is_done": false
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;

    // Expect
    let mut db = app_state.db.acquire().await?;
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
            .fetch_one(&mut *db)
            .await
            .unwrap();
    assert_eq!(data.0, 1);
    assert_eq!(data.1, "first todo".to_string());
    assert_eq!(data.2, 0);
    Ok(())
}

#[sqlx::test]
async fn update_todo(pool: SqlitePool) -> sqlx::Result<()> {
    // Given
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state.clone());
    let cli = TestClient::new(app);
    let json_payload = json!({
        "todo": "first todo",
        "is_done": false
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;
    resp.assert_status_is_ok();
    let mut db = app_state.db.acquire().await?;

    // When 1
    let resp = cli
        .put(format!("/api/todo/{}", 1))
        .body_json(&json!({
            "todo": "updated_todo",
            "is_done": true
        }))
        .send()
        .await;

    // Expect 1
    resp.assert_status_is_ok();
    resp.assert_json(json!({
        "id": 1,
        "todo": "updated_todo",
        "is_done": true
    }))
    .await;
    let data: (i32, String, i32) =
        sqlx::query_as("SELECT id, todo, is_done FROM todo WHERE id = ?")
            .bind(1)
            .fetch_one(&mut *db)
            .await
            .unwrap();
    assert_eq!(data.0, 1);
    assert_eq!(data.1, "updated_todo".to_string());
    assert_eq!(data.2, 1);

    // When 2
    let resp = cli
        .put(format!("/api/todo/{}", 2))
        .body_json(&json_payload)
        .send()
        .await;

    // Expect 2
    resp.assert_status(StatusCode::NOT_FOUND);
    resp.assert_json(json!({
        "message": "todo with id 2 not found"
    }))
    .await;
    Ok(())
}

#[sqlx::test]
async fn delete_todo(pool: SqlitePool) -> sqlx::Result<()> {
    // Given
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state.clone());
    let cli = TestClient::new(app);
    let json_payload = json!({
        "todo": "first todo",
        "is_done": false
    });
    let resp = cli.post("/api/todo").body_json(&json_payload).send().await;
    resp.assert_status_is_ok();
    let mut db = app_state.db.acquire().await?;

    // When 1
    let resp = cli.delete(format!("/api/todo/{}", 1)).send().await;

    // Expect 1
    resp.assert_status_is_ok();
    let data: Option<(i32, String, i32)> =
        sqlx::query_as("SELECT id, todo, is_done FROM todo WHERE id = ?")
            .bind(1)
            .fetch_optional(&mut *db)
            .await
            .unwrap();
    assert!(data.is_none());
    Ok(())
}
