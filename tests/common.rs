use entity::prelude::*;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbConn, Set};
use uuid::Uuid;

#[allow(dead_code)]
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

pub async fn create_three_test_tasks(db: &DbConn) -> Vec<String>{
    let mut tasks_ids: Vec<String> = Vec::with_capacity(3);
    for n in 1..=3 {
        let task_id = Uuid::new_v4().to_string();
        let task_title = format!("Task title 00{}", n);
        let todo: TaskActiveModel = TaskActiveModel {
            id: Set(task_id.to_owned()),
            title: Set(task_title.to_owned()),
            ..Default::default()
        };
        todo.insert(db)
            .await
            .expect("Failed to insert task in database");
        tasks_ids.push(task_id);
    }

    tasks_ids.sort();

    tasks_ids
}
