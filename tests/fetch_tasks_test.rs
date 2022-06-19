use migration::DbErr;
use todo_rs::prelude::*;

mod common;
use common::*;

#[tokio::test]
async fn fetch_all_tasks_test() -> Result<(), DbErr> {
    let db = get_db_conn().await;
    // add test tasks in the database
    let tasks_ids = create_three_test_tasks(&db).await;
    
    let all_tasks = fetch_all_tasks(&db).await;

    let mut all_tasks_ids: Vec<String> = Vec::with_capacity(3);

    
    for task in all_tasks {
        all_tasks_ids.push(task.id)
    }
    
    all_tasks_ids.sort();
    assert_eq!(tasks_ids, all_tasks_ids);

    Ok(())
}