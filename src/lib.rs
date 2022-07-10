use chrono::{Local, SubsecRound};
use entity::task;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ColumnTrait, Database, DbConn, EntityTrait, QueryFilter, Set};

pub async fn get_db_conn() -> Result<DbConn, DbErr> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}

pub async fn create_task(db: &DbConn, id:String, title: String) -> Result<task::Model, DbErr> {
    let todo = task::ActiveModel {
        id: Set(id),
        title: Set(title),
        created_at: Set(Local::now().round_subsecs(0).format("%F").to_string()),
        ..Default::default()
    };
    let todo = todo.insert(db).await;
    todo
}

pub async fn update_task_is_done(
    db: &DbConn,
    task_id: String,
    is_done: bool,
) -> Result<task::Model, DbErr> {
    let todo = task::Entity::find_by_id(task_id).one(db).await?;
    let mut todo: task::ActiveModel = todo.unwrap().into();
    todo.is_done = Set(if is_done { 1 } else { 0 });
    todo.finished_at = Set(if is_done {
        Some(Local::today().format("%F").to_string())
    } else {
        None
    });
    let todo = todo.update(db).await;
    todo
}

pub async fn get_all_undone_tasks(db: &DbConn) -> Result<Vec<task::Model>, DbErr> {
    let todos = task::Entity::find()
        .filter(task::Column::IsDone.eq(0))
        .all(db)
        .await;
    todos
}

pub async fn get_all_done_tasks_for_today(db: &DbConn) -> Result<Vec<task::Model>, DbErr> {
    let today = Local::today().format("%F").to_string();
    let todos = task::Entity::find()
        .filter(task::Column::FinishedAt.eq(today))
        .filter(task::Column::IsDone.eq(1))
        .all(db)
        .await;
    todos
}
