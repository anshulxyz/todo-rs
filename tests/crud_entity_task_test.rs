        use migration::{Migrator, MigratorTrait, DbErr};
use sea_orm::{Database, DbConn, Set, ActiveModelTrait, EntityTrait, PaginatorTrait, ModelTrait};
use uuid::Uuid;
use entity::task;

async fn setup() -> DbConn {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None).await.expect("Failed to run migrations for tests");

    db
}

#[tokio::test]
async fn crud_test() -> Result<(), DbErr> {

    let db = setup().await;

    let task_id = Uuid::new_v4().to_string();
    let task_title = "Task Title 001".to_string();
    let todo: task::ActiveModel = task::ActiveModel {
        id: Set(task_id.to_owned()),
        title: Set(task_title.to_owned()),
        ..Default::default()
    };

    // CREATE
    let todo: task::Model = todo.insert(&db).await?;

    assert_eq!(task_id, todo.id);
    assert_eq!(1, task::Entity::find().count(&db).await?);
    
    // READ
    assert_eq!(task_title, todo.title);

    // DELETE
    let result = todo.delete(&db).await?;
    println!("Deleted: {:?}", result);
    let todo = task::Entity::find_by_id(task_id.to_owned()).one(&db).await?;
    assert_eq!(None, todo);
    assert_eq!(0, task::Entity::find().count(&db).await?);
    
    Ok(())
}
