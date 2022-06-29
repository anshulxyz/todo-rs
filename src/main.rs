use cursive::{
    traits::Nameable,
    views::{Checkbox, Dialog, EditView, LinearLayout, ListView, TextView},
    Cursive,
};
use migration::DbErr;
use todo_rs::{
    get_all_done_tasks_for_today, get_all_undone_tasks, get_db_conn, update_task_is_done, create_task,
};

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
    let mut list_view = ListView::new();

    for todo in todos {
        let mut checkbox = Checkbox::new();
        checkbox.set_checked(todo.is_done != 0);
        checkbox.set_on_change(move |s, checked| {
            // let moved_db = db;
            let task_id = todo.id.to_owned();
            tokio::spawn(async move {
                let closure_db = get_db_conn().await.expect("Failed to created database connection for closure");
                let _todo = update_task_is_done(&closure_db, task_id, checked)
                    .await
                    .expect("Failed to change status of the task");
            });
            s.add_layer(Dialog::info(format!(
                "The task was marked: {}",
                if checked { "DONE" } else { "UNDONE`" }
            )));
        });
        let title = todo.title.to_owned();
        let title = TextView::new(title);
        let linear_layout = LinearLayout::horizontal().child(checkbox).child(title);
        list_view.add_child("-", linear_layout);
    }

    siv.add_layer(list_view);

    siv.add_global_callback('a', |s| {
        let dialog = Dialog::new()
            .title("Add new task!")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .on_submit(show_popup)
                    .with_name("add_edit_view"),
            )
            .button("Ok", |s| {
                let todo = s
                    .call_on_name("add_edit_view", |view: &mut EditView| view.get_content())
                    .unwrap();
                show_popup(s, &todo);
            });
        s.add_layer(dialog);
    });

    siv.run();

    Ok(())
}

fn show_popup(s: &mut Cursive, todo: &str) {
    if todo.is_empty() {
        s.add_layer(Dialog::info("Please enter a task!"));
    } else {
        let todo = todo.to_owned();
        tokio::spawn(async move {
            let db = get_db_conn().await.expect("Failed to created database connection for closure");
            let _todo = create_task(&db, todo.as_str()).await;
        });
        s.pop_layer();
        s.add_layer(Dialog::info("Task added to the database"));
    }
}
