use poem::listener::TcpListener;
use sqlx::SqlitePool;
use tokio::process::Command;
use try_poem::{init_openapi_routes, AppState};

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:./sqlite.db?mode=rwc")
        .await
        .unwrap();
    let _ = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .status()
        .await
        .unwrap();
    let app_state = AppState { db: pool };
    let app = init_openapi_routes(app_state);

    println!("run on 0.0.0.0:3000");
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
        .unwrap()
}
