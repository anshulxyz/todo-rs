use cursive::{
    traits::Nameable,
    views::{Checkbox, Dialog, LinearLayout, ListView, TextView},
};
use migration::DbErr;
use todo_rs::{create_task, get_all_done_tasks_for_today, get_all_undone_tasks, get_db_conn};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // initialize database
    let db = get_db_conn()
        .await
        .expect("Failed to make a database connection");

    // initialize cursive
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    // fetch data, merge the all the todos to display for today
    let mut todos = get_all_undone_tasks(&db).await?;
    let mut done_todos_for_today = get_all_done_tasks_for_today(&db).await?;
    todos.append(&mut done_todos_for_today);

    // create a list of checkbox-views, populate them with todos
    let mut undone_list_view = ListView::new();

    for (index, todo) in todos.iter().enumerate() {
        let task_id = todo.id.to_owned();
        let mut checkbox = Checkbox::new().on_change(move |s, checked| {
            s.add_layer(Dialog::info(format!(
                "The task '{}' was marked {}",
                task_id,
                if checked { "done" } else { "undone" }
            )));
        });
        checkbox.set_checked(todo.is_done != 0);
        let title = todo.title.to_owned();
        let title = TextView::new(title);
        let linear_layout = LinearLayout::horizontal().child(checkbox).child(title);
        undone_list_view.add_child(format!("{:?}", index).as_str(), linear_layout);
    }

    siv.add_layer(undone_list_view);

    siv.run();

    Ok(())
}
