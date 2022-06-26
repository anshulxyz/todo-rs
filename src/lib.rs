use chrono::{Local, SubsecRound};
use entity::task;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn create_task(db: &DbConn, title: &str) -> Result<task::Model, DbErr> {
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
) -> Result<task::Model, DbErr> {
    let todo = task::Entity::find_by_id(task_id).one(db).await?;
    let mut todo: task::ActiveModel = todo.unwrap().into();
    todo.is_done = Set(if is_done { 1 } else { 0 });
    let todo = todo.update(db).await;
    todo
}

pub async fn get_all_undone_tasks(db: &DbConn) -> Result<Vec<task::Model>, DbErr> {
    let todos = task::Entity::find()
        .filter(task::Column::IsDone.eq(0))
        .all(db)
        .await;
    todos
}
