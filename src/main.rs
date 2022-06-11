use cursive::views::{Checkbox, LinearLayout, ListView, TextView};
use entity::task;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use std::error::Error;

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

    for todo in all_undone_tasks {
        let task_id = todo.id.to_owned();
        let child_view = LinearLayout::horizontal()
            .child(Checkbox::new().on_change(move |s, checked| {
                println!("{:?}", task_id)
            }))
            .child(TextView::new(&todo.title));
        list.add_child("1", child_view);
    }

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(list);

    siv.run();

    Ok(())
}
