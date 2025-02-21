use std::{path::PathBuf, sync::Arc};

use poem::{
    http::StatusCode,
    test::{TestClient, TestForm, TestFormField},
};
use serde_json::json;
use serde_json::Value::Null;
use sqlx::SqlitePool;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{init_openapi_routes, AppState};

#[tokio::test]
async fn hello() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli.get("/api/example/hello").send().await;

    // Expect
    resp.assert_status(StatusCode::OK);
    resp.assert_text("hello").await;
}

#[tokio::test]
async fn path_query() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/example/path-query/hello")
        .query("query_1", &"world")
        .query("query_2", &10)
        .send()
        .await;

    // Expect
    resp.assert_status(StatusCode::OK);
    resp.assert_json(json!({
        "path": "hello",
        "query_1": "world",
        "query_2": 10
    }))
    .await;
}

#[tokio::test]
async fn path_query_nullable() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli.get("/api/example/path-query/bro").send().await;

    // Expect
    resp.assert_status(StatusCode::OK);
    resp.assert_json(json!({
        "path": "bro",
        "query_1": Null,
        "query_2": Null
    }))
    .await;
}

#[tokio::test]
async fn multiple_response_ok() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/example/multiple-response")
        .query("status", &200)
        .send()
        .await;

    // Expect
    resp.assert_status(StatusCode::OK);
    resp.assert_json(json!({
        "data": "some data"
    }))
    .await;
}

#[tokio::test]
async fn multiple_response_bad_request() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/example/multiple-response")
        .query("status", &400)
        .send()
        .await;

    // Expect
    resp.assert_status(StatusCode::BAD_REQUEST);
    resp.assert_json(json!({
        "validation_error": "some validataion error"
    }))
    .await;
}

#[tokio::test]
async fn multiple_response_internal_server_error() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/example/multiple-response")
        .query("status", &500)
        .send()
        .await;

    // Expect
    resp.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
    resp.assert_json(json!({
        "error": "some error"
    }))
    .await;
}

#[tokio::test]
async fn multiple_response_uncheck_status_code() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/example/multiple-response")
        .query("status", &412)
        .send()
        .await;

    // Expect
    resp.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    resp.assert_json(json!({
        "validation_error": "invalid status = 412"
    }))
    .await;
}

#[tokio::test]
async fn json_payload_and_response() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let payload = json!({
        "key1": "hello",
        "key2": 1,
        "key3": true
    });
    let resp = cli
        .post("/api/example/json")
        .body_json(&payload)
        .send()
        .await;

    // Expect
    // println!("{:?}", resp.0.into_body().into_string().await);
    resp.assert_status(StatusCode::OK);
    resp.assert_json(payload).await;
}

#[tokio::test]
async fn form_payload() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);
    // hello file
    let mut file_path_hello = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path_hello.push("files/hello.txt");
    let mut file_hello = File::open(file_path_hello).await.unwrap();
    let mut contents_hello = vec![];
    file_hello.read_to_end(&mut contents_hello).await.unwrap();

    // foo file
    let mut file_path_foo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path_foo.push("files/foo.txt");
    let mut file_foo = File::open(file_path_foo).await.unwrap();
    let mut contents_foo = vec![];
    file_foo.read_to_end(&mut contents_foo).await.unwrap();

    // bar file
    let mut file_path_bar = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path_bar.push("files/bar.txt");
    let mut file_bar = File::open(file_path_bar).await.unwrap();
    let mut contents_bar = vec![];
    file_bar.read_to_end(&mut contents_bar).await.unwrap();

    // When
    let resp = cli
        .post("/api/example/form")
        .multipart(
            TestForm::new()
                .text("key1", "key1")
                .text("key2", "1")
                .text("key 5", "3")
                .field(
                    TestFormField::bytes(contents_hello)
                        .name("file")
                        .filename("hello.txt"),
                )
                .bytes("files", contents_foo)
                .bytes("files", contents_bar),
        )
        .send()
        .await;

    resp.assert_status(StatusCode::OK);
    resp.assert_json(json!({
        "key1": "key1",
        "key2": 1,
        "key3": Null,
        "key4": Null,
        "key5": 3,
        "file": "hello.txt",
        "files": vec!["", ""]
    }))
    .await;
}

#[tokio::test]
async fn auth_example() {
    // Given
    let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
    let app_state = Arc::new(AppState { db: pool });
    let app = init_openapi_routes(app_state);
    let cli = TestClient::new(app);

    // When
    let resp = cli
        .get("/api/example/auth")
        .header("X-API-Key", "boo")
        .send()
        .await;

    // Expect
    resp.assert_text("boo").await;
}
