use sea_orm::{DbConn, EntityTrait};
use entity::prelude::*;

/// Fetch all the tasks
pub async fn fetch_all_tasks(db: &DbConn) -> Vec<TaskModel> {
    Task::find().all(db).await.expect("Failed to fetch all tasks")
}