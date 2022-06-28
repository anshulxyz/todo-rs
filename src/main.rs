use cursive::{
    traits::Nameable,
    views::{Checkbox, LinearLayout, ListView, TextView},
};
use migration::DbErr;
use todo_rs::{create_task, get_all_done_tasks_for_today, get_all_undone_tasks, get_db_conn};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // initialize database
    let db = get_db_conn()
        .await
        .expect("Failed to make a database connection");

    // for i in 0..5 {
    //     let _todo = create_task(&db, "some title");
    // }

    // initialize cursive
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    // fetch data
    let undone_todos = get_all_undone_tasks(&db).await?;
    let done_todos = get_all_done_tasks_for_today(&db).await?;

    // create a list of checkbox-views, populate them with todos
    let mut undone_list_view = ListView::new();
    let mut index = 0;

    for todo in undone_todos {
        let task_id = todo.id.to_owned();
        let checkbox = Checkbox::new().on_change(move |s, checked| {
            println!("{:?}", task_id);
        });
        let title = TextView::new(todo.title);
        let linear_layout = LinearLayout::horizontal().child(checkbox).child(title);
        undone_list_view.add_child(format!("{:?}", index).as_str(), linear_layout);
        index += 1;
    }

    siv.add_layer(undone_list_view);

    let mut done_list_view = ListView::new();

    for todo in done_todos {
        let task_id = todo.id.to_owned();
        let checkbox = Checkbox::new().on_change(move |s, checked| {
            println!("{:?}, {:?}", checked, task_id);
        });
        let title = TextView::new(todo.title);
        let linear_layout = LinearLayout::horizontal().child(checkbox).child(title);
        done_list_view.add_child(format!("{:?}", index).as_str(), linear_layout);
        index += 1;
    }

    siv.add_layer(done_list_view);

    siv.run();

    Ok(())
}
