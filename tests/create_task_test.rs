use entity::task;
use migration::DbErr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use todo_rs::prelude::*;

mod common;
use common::*;

/// Test creation of tasks
#[tokio::test]
async fn create_task_test() -> Result<(), DbErr> {
    let db = get_db_conn().await;

    // rust optional
    let created_task_id = create_task(&db, "title", Some("description"), None).await?;

    let found_task = task::Entity::find()
        .filter(task::Column::Title.eq("title"))
        .one(&db)
        .await?;

    assert_eq!(found_task.as_ref().unwrap().title, "title");
    assert_eq!(
        found_task.as_ref().unwrap().text.as_ref().unwrap(),
        "description"
    );
    assert!(!created_task_id.is_empty());

    Ok(())
}
