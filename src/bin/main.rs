use poem::listener::TcpListener;
use sqlx::SqlitePool;
use tokio::process::Command;
use tracing::Level;
use try_poem::{init_openapi_routes, AppState};

#[tokio::main]
async fn main() {
    let log_level = Level::DEBUG;
    // tracing must be run on main function
    // connot put on if or other function
    // tracing only support one writer
    // (maybe this will help https://github.com/tokio-rs/tracing/issues/971)
    // https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-multiple-writers.rs
    // I don't know why this happen???

    // Logging to File
    // let file_appender = tracing_appender::rolling::daily("./logs", "app.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // tracing_subscriber::fmt()
    //     .with_writer(non_blocking)
    //     .with_max_level(log_level)
    //     .init();

    // Logging to Console
    tracing_subscriber::fmt().with_max_level(log_level).init();

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
