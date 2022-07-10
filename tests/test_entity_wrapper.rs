use std::ops::Sub;

use entity::task;
use migration::DbErr;
use todo_rs::{
    create_task, get_all_done_tasks_for_today, get_all_undone_tasks, get_db_conn,
    update_task_is_done,
};

use chrono::{Duration, Local, SubsecRound};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use uuid::Uuid;

#[tokio::test]
async fn test_create_task() -> Result<(), DbErr> {
    // given
    let task_title = "Task Title".to_string();
    let db = get_db_conn().await?;
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(0, count);

    // when
    let uuid = Uuid::new_v4().to_string();
    let todo = create_task(&db, uuid, task_title.to_owned()).await.unwrap();

    // then
    assert_eq!(task_title, todo.title);
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(1, count);
    Ok(())
}

#[tokio::test]
async fn test_update_task_is_done() -> Result<(), DbErr> {
    // given we have created a task
    let db = get_db_conn().await?;
    let task_title = "Task title".to_string();
    let todo = task::ActiveModel {
        id: Set(Uuid::new_v4().to_owned().to_string()),
        title: Set(task_title.to_owned()),
        created_at: Set(Local::now().round_subsecs(0).format("%F").to_string()),
        ..Default::default()
    };
    let todo: task::Model = todo.insert(&db).await.unwrap();
    assert_eq!(todo.is_done, 0);
    // when we update its is_done status to 1
    let todo = update_task_is_done(&db, todo.id, true).await?;
    // then
    assert_eq!(todo.title, task_title);
    assert!(todo.finished_at.is_some());
    assert_eq!(todo.is_done, 1);
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(1, count);

    Ok(())
}

#[tokio::test]
async fn test_get_all_undone_tasks_when_none_exist() -> Result<(), DbErr> {
    // given we there are no tasks in database
    let db = get_db_conn().await?;

    // when I fetch all the undone takss
    let all_undone_tasks: Vec<task::Model> = get_all_undone_tasks(&db).await?;

    // then we should get an empty vec
    assert_eq!(all_undone_tasks.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_get_all_undone_tasks() -> Result<(), DbErr> {
    // given we have created three tasks, of which one is marked done
    let db = get_db_conn().await?;
    for i in 0..=2 {
        let todo = task::ActiveModel {
            id: Set(Uuid::new_v4().to_owned().to_string()),
            title: Set(format!("Task title 00{}", i)),
            created_at: Set(Local::now().round_subsecs(0).format("%F").to_string()),
            is_done: Set(if i == 0 { 1 } else { 0 }),
            ..Default::default()
        };
        let _todo = todo.insert(&db).await;
    }
    // when I fetch all the undone takss
    let all_undone_tasks: Vec<task::Model> = get_all_undone_tasks(&db).await?;

    // then I should only get two tasks whose status is undone
    assert_eq!(all_undone_tasks.len(), 2);
    for todo in all_undone_tasks {
        assert_eq!(todo.is_done, 0);
    }
    Ok(())
}

#[tokio::test]
async fn test_get_all_done_tasks_for_today() -> Result<(), DbErr> {
    // given there are four tasks, in database, ONLY one was finished today, and three were finished yesterday
    let db = get_db_conn().await?;
    for i in 0..=3 {
        let todo = task::ActiveModel {
            id: Set(Uuid::new_v4().to_owned().to_string()),
            title: Set(format!("Task title 00{}", i)),
            created_at: Set(Local::today()
                .sub(Duration::days(3))
                .format("%F")
                .to_string()),
            finished_at: Set(Some(
                Local::today()
                    .sub(Duration::days(if i == 0 { 0 } else { 1 }))
                    .format("%F")
                    .to_string(),
            )),
            is_done: Set(1),
        };
        let _todo = todo.insert(&db).await;
    }

    // when we fetch all the done tasks for "today"
    let todos: Vec<task::Model> = get_all_done_tasks_for_today(&db).await?;

    // then we should get an empty vec
    assert_eq!(todos.len(), 1);

    // etc, check there are three tasks finished yesterday
    let yesterday = Local::today()
        .sub(Duration::days(1))
        .format("%F")
        .to_string();
    let count = task::Entity::find()
        .filter(task::Column::FinishedAt.eq(yesterday))
        .all(&db)
        .await?;
    assert_eq!(3, count.len());

    Ok(())
}

#[tokio::test]
async fn test_set_and_unset_status_of_task() -> Result<(), DbErr> {
    // given we have a task
    let task_title = "CHanged task".to_string();
    let db = get_db_conn().await?;
    let count = task::Entity::find().count(&db).await.unwrap_or(0);
    assert_eq!(0, count);

    let uuid = Uuid::new_v4().to_string();
    let todo = create_task(&db, uuid, task_title).await.unwrap();
    assert!(todo.finished_at.is_none());

    // when we set it's status to done, and then to undone
    let todo = update_task_is_done(&db, todo.id, true).await?;
    assert_eq!(todo.is_done, 1);
    assert!(todo.finished_at.is_some());

    // when we change it's status again to false
    let todo = update_task_is_done(&db, todo.id, false).await?;
    assert_eq!(todo.is_done, 0);

    // then it's `finished_at` field should be None/NULL
    assert!(todo.finished_at.is_none());

    Ok(())
}
