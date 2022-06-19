use entity::task;
use entity::prelude::*;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, DbConn, EntityTrait, ModelTrait, PaginatorTrait, Set};
use uuid::Uuid;

mod common;
use common::get_db_conn;

#[tokio::test]
async fn crud_test() -> Result<(), DbErr> {
    let db = get_db_conn().await;

    let task_id = Uuid::new_v4().to_string();
    let task_title = "Task Title 001".to_string();
    let todo: TaskActiveModel = TaskActiveModel {
        id: Set(task_id.to_owned()),
        title: Set(task_title.to_owned()),
        ..Default::default()
    };

    // CREATE
    let todo: TaskModel = todo.insert(&db).await?;

    assert_eq!(task_id, todo.id);
    assert_eq!(1, Task::find().count(&db).await?);

    // READ
    assert_eq!(task_title, todo.title);

    // DELETE
    let result = todo.delete(&db).await?;
    println!("Deleted: {:?}", result);
    let todo = Task::find_by_id(task_id.to_owned())
        .one(&db)
        .await?;
    assert_eq!(None, todo);
    assert_eq!(0, Task::find().count(&db).await?);

    Ok(())
}
