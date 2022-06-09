use std::error::Error;

use entity::task;

use migration::{Migrator, MigratorTrait};
use sea_orm::EntityTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = sea_orm::Database::connect("sqlite://tasks.db?mode=rwc").await?;
    Migrator::up(&connection, None).await?;

    // fetch an item from database
    let todo: task::Model = task::Entity::find().one(&connection).await.expect("Error fetching verse").unwrap();

    println!("{:?}", todo);

    Ok(())
}