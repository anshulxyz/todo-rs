use chrono::Utc;
use entity::task;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, PaginatorTrait, Set};
use uuid::Uuid;

mod common;
use common::get_db_conn;

/// Testing the SeaORM Entities
#[tokio::test]
async fn crud_test() -> Result<(), DbErr> {
    let db = get_db_conn().await;

    let task_id = Uuid::new_v4().to_string();
    let task_title = "Task Title 001".to_string();
    let todo = task::ActiveModel {
        id: Set(task_id.to_owned()),
        title: Set(task_title.to_owned()),
        created_at: Set(Utc::now().to_string()),
        ..Default::default()
    };

    // CREATE
    let todo: task::Model = todo.insert(&db).await?;

    assert_eq!(task_id, todo.id);
    assert_eq!(1, task::Entity::find().count(&db).await?);

    // READ
    assert_eq!(task_title, todo.title);

    // DELETE
    let result = todo.delete(&db).await?;
    println!("Deleted: {:?}", result);
    let todo = task::Entity::find_by_id(task_id.to_owned()).one(&db).await?;
    assert_eq!(None, todo);
    assert_eq!(0, task::Entity::find().count(&db).await?);

    Ok(())
}
