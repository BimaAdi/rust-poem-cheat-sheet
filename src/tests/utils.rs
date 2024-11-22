use std::path::Path;

use tokio::process::Command;

pub async fn setup_database(db_url: &str) {
    let db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(&db_url);
    let _ = Command::new("rm")
        .arg(db_path.as_os_str())
        .status()
        .await
        .unwrap();
    let _ = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .arg("--database-url")
        .arg(format!("sqlite:./{}?mode=rwc", &db_url))
        .status()
        .await
        .unwrap();
}
