/// Tests for wrappers around entity for ease of operation
use sea_orm::{EntityTrait, PaginatorTrait};
use todo_rs::create_task;
use entity::task;

mod common;
use common::get_db_conn;
use migration::DbErr;

#[tokio::test]
async fn test_create_task() -> Result<(), DbErr> {
    let task_title = "Task Title";
    let db = get_db_conn().await;
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(0, count);
    let todo = create_task(&db, task_title).await.unwrap();
    assert_eq!(task_title, todo.title);
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(1, count);
    Ok(())
}
