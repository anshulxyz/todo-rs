use chrono::{Local, SubsecRound};
use entity::task;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};
use uuid::Uuid;

pub async fn create_task(db: &DbConn, title: &str) -> Result<task::Model, migration::DbErr> {
    let todo = task::ActiveModel {
        id: Set(Uuid::new_v4().to_owned().to_string()),
        title: Set(title.to_string()),
        created_at: Set(Local::now()
            .round_subsecs(0)
            .format("%F %H:%M:%S")
            .to_string()),
        ..Default::default()
    };
    let todo = todo.insert(db).await;
    todo
}

pub async fn update_task_is_done(
    db: &DbConn,
    task_id: String,
    is_done: bool,
) -> Result<task::Model, migration::DbErr> {
    let todo = task::Entity::find_by_id(task_id).one(db).await?;
    let mut todo: task::ActiveModel = todo.unwrap().into();
    todo.is_done = Set(if is_done { 1 } else { 0 });
    let todo = todo.update(db).await?;
    Ok(todo)
}
