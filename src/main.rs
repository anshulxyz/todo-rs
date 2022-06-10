use cursive::views::{ListView, TextView};
use entity::task;
use std::error::Error;
use uuid::Uuid;

use chrono::Utc;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = sea_orm::Database::connect("sqlite://tasks.db?mode=rwc").await?;
    Migrator::up(&db, None).await?;

    let all_undone_tasks = task::Entity::find()
        .filter(task::Column::IsDone.eq(0))
        .all(&db)
        .await
        .expect("Error fetching verse");
    println!("{:?}", all_undone_tasks);

    let mut list = ListView::new();

    for (index, todo) in all_undone_tasks.iter().enumerate() {
        let child_view = TextView::new(&todo.title);
        list.add_child(&index.to_string(), child_view);
    }

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(list);

    siv.run();

    Ok(())
}
