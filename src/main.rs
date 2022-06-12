use cursive::views::{Checkbox, Dialog, LinearLayout, ListView, TextView};
use entity::task;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use std::{error::Error, fmt::format};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = sea_orm::Database::connect("sqlite://tasks.db?mode=rwc").await?;
    Migrator::up(&db, None).await?;

    let all_undone_tasks = task::Entity::find()
        // .filter(task::Column::IsDone.eq(0))
        .all(&db)
        .await
        .expect("Error fetching verse");

    let mut list = ListView::new();

    for (index, todo) in all_undone_tasks.iter().enumerate() {
        let is_checked: bool = todo.is_done == 1;
        let task_id = todo.id.to_owned();
        let checkbox_view =
            Checkbox::new()
                .with_checked(is_checked)
                .on_change(move |s, checked| {
                    let is_marked_done = if checked { "Done" } else { "Undone" };
                    s.add_layer(Dialog::info(format!(
                        "The task {} is marked: {:?}",
                        task_id, is_marked_done
                    )));
                });

        let child_view = LinearLayout::horizontal()
            .child(checkbox_view)
            .child(TextView::new(&todo.title));
        list.add_child(index.to_string().as_str(), child_view);
    }

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(list);

    siv.run();

    Ok(())
}
