use entity::task;
use std::error::Error;
use uuid::Uuid;

use chrono::Utc;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, EntityTrait, Set};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = sea_orm::Database::connect("sqlite://tasks.db?mode=rwc").await?;
    Migrator::up(&db, None).await?;

    // insert a task in data
    let todo = task::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        title: Set("Title".to_string()),
        is_done: Set(0),
        created_at: Set(Utc::now().naive_utc().to_string()),
        text: NotSet,
        finished_at: NotSet,
        due_at: NotSet,
    };

    let todo = todo.insert(&db).await?;
    println!("INSERT {:?}", todo);

    // fetch an item from database
    let todos = task::Entity::find()
        .all(&db)
        .await
        .expect("Error fetching verse");
    println!("FETCH {:?}", todos);

    Ok(())
}
