use chrono::Utc;
use entity::task;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, DbConn, Set};
use uuid::Uuid;

pub async fn create_task(
    db: &DbConn,
    title: &str,
    text: Option<&str>,
    due_date: Option<&str>,
) -> Result<String, DbErr> {
    let task_active_model = task::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        title: Set(title.to_string()),
        text: Set(Some(text.unwrap_or("default").to_string())),
        created_at: Set(Utc::now().to_string()),
        due_at: Set(Some(due_date.unwrap_or("").to_string())),
        ..Default::default()
    };
    let task_model = task_active_model.insert(db).await?;

    Ok(task_model.id)
}
