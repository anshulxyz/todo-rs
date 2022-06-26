mod common;
use common::get_db_conn;

use entity::task;
use migration::DbErr;
use todo_rs::{create_task, get_all_undone_tasks, update_task_is_done};

use chrono::{Local, SubsecRound};
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

#[tokio::test]
async fn test_create_task() -> Result<(), DbErr> {
    // given
    let task_title = "Task Title";
    let db = get_db_conn().await;
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(0, count);

    // when
    let todo = create_task(&db, task_title).await.unwrap();

    // then
    assert_eq!(task_title, todo.title);
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(1, count);
    Ok(())
}

#[tokio::test]
async fn test_update_task_is_done() -> Result<(), DbErr> {
    // given we have created a task
    let db = get_db_conn().await;
    let task_title = "Task title".to_string();
    let todo = task::ActiveModel {
        id: Set(Uuid::new_v4().to_owned().to_string()),
        title: Set(task_title.to_owned()),
        created_at: Set(Local::now()
            .round_subsecs(0)
            .format("%F %H:%M:%S")
            .to_string()),
        ..Default::default()
    };
    let todo: task::Model = todo.insert(&db).await.unwrap();
    assert_eq!(todo.is_done, 0);
    // when we update its is_done status to 1
    let todo = update_task_is_done(&db, todo.id, true).await?;
    // then
    assert_eq!(todo.title, task_title);
    assert_eq!(todo.is_done, 1);
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(1, count);

    Ok(())
}

#[tokio::test]
async fn test_get_all_undone_tasks() -> Result<(), DbErr> {
    // given we have created three tasks, of which one is marked done
    let db = get_db_conn().await;
    for i in 0..=2 {
        let todo = task::ActiveModel {
            id: Set(Uuid::new_v4().to_owned().to_string()),
            title: Set(format!("Task title 00{}", i)),
            created_at: Set(Local::now()
                .round_subsecs(0)
                .format("%F %H:%M:%S")
                .to_string()),
            is_done: Set(if i == 0 { 1 } else { 0 }),
            ..Default::default()
        };
        let _todo = todo.insert(&db).await;
    }
    // when I fetch all the undone takss
    let all_undone_tasks: Vec<task::Model> = get_all_undone_tasks(&db).await?;

    // then I should only get two tasks whose status is undone
    assert_eq!(all_undone_tasks.len(), 2);
    Ok(())
}
