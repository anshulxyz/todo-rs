use cursive::{
    views::{Checkbox, Dialog, LinearLayout, ListView, TextView},
};
use entity::task;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, DatabaseConnection, ActiveModelTrait};
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
                    update_is_done_status(&task_id, checked);
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

async fn update_is_done_status(task_id: &String, is_done: bool) -> Result<(), Box<dyn Error>> {
    let task_id = task_id.to_owned();
    let db = sea_orm::Database::connect("sqlite://tasks.db?mode=rwc").await?;
    let task_item = task::Entity::find_by_id(task_id).one(&db).await?;
    let mut task_item: task::ActiveModel = task_item.unwrap().into();
    task_item.is_done = Set(if is_done {1} else {0});
    task_item.update(&db).await?;
    Ok(())
}