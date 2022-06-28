use chrono::{Local, SubsecRound};
use todo_rs::get_db_conn;

use entity::task;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, PaginatorTrait, Set, Unchanged};
use uuid::Uuid;

/// Testing the SeaORM Entities
#[tokio::test]
async fn test_crud_entity() -> Result<(), DbErr> {
    let db = get_db_conn().await;

    let task_id = Uuid::new_v4().to_string();
    let task_title = "Task Title 001".to_string();
    let todo = task::ActiveModel {
        id: Set(task_id.to_owned()),
        title: Set(task_title.to_owned()),
        created_at: Set(Local::now().round_subsecs(0).format("%F").to_string()),
        ..Default::default()
    };

    // CREATE
    let todo: task::Model = todo.insert(&db).await?;

    assert_eq!(task_id, todo.id);
    assert_eq!(1, task::Entity::find().count(&db).await?);

    // READ
    assert_eq!(task_title, todo.title);

    //UPDATE
    let todo = task::Entity::find_by_id(task_id.to_owned())
        .one(&db)
        .await?;
    let mut todo: task::ActiveModel = todo.unwrap().into();
    assert_eq!(todo.is_done, Unchanged(0));
    todo.is_done = Set(1);
    let todo: task::Model = todo.update(&db).await?;
    assert_eq!(todo.is_done, 1);

    // DELETE
    let result = todo.delete(&db).await?;
    println!("Deleted: {:?}", result);
    let todo = task::Entity::find_by_id(task_id.to_owned())
        .one(&db)
        .await?;
    assert_eq!(None, todo);
    assert_eq!(0, task::Entity::find().count(&db).await?);

    Ok(())
}
