/// Tests for wrappers around entity for ease of operation
use todo_rs::create_task;

mod common;
use common::get_db_conn;
use migration::DbErr;

#[tokio::test]
async fn test_create_task() -> Result<(), DbErr> {
    let task_title = "Task Title";
    let db = get_db_conn().await;
    let todo = create_task(&db, task_title).await.unwrap();
    assert_eq!(task_title, todo.title);
    Ok(())
}
