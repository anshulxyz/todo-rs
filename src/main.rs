use cursive::views::{Checkbox, LinearLayout, ListView, TextView};
use entity::task;
use std::error::Error;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter, Set};

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
        let child_view = LinearLayout::horizontal()
            .child(Checkbox::new().on_change(|s, checked| {
            }))
            .child(TextView::new(&todo.title));
        list.add_child(&index.to_string(), child_view);
    }

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(list);

    siv.run();

    Ok(())
}
