use std::error::Error;

use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = sea_orm::Database::connect("sqlite://tasks.db").await?;
    Migrator::up(&connection, None).await?;
    Ok(())
}