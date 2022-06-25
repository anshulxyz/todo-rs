use entity::task;
use sea_orm::{DbConn, Set, ActiveModelTrait};
use uuid::Uuid;
use chrono::{Local, SubsecRound};


pub async fn create_task(db: &DbConn, title: &str) -> Result<task::Model, migration::DbErr> {
    let todo = task::ActiveModel {
        id: Set(Uuid::new_v4().to_owned().to_string()),
        title: Set(title.to_string()),
        created_at: Set(Local::now().round_subsecs(0).format("%F %H:%M:%S").to_string()),
        ..Default::default()
    };
    let todo = todo.insert(db).await;
    todo
}
