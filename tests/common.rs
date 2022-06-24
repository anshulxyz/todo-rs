#![allow(dead_code)]

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

pub async fn get_db_conn() -> DbConn {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    db
}
