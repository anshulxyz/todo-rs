use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Task::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Task::Title).string().not_null())
                    .col(ColumnDef::new(Task::IsDone).boolean().not_null().default(0))
                    .col(ColumnDef::new(Task::CreatedAt).date().not_null())
                    .col(ColumnDef::new(Task::FinishedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Task {
    Table,
    Id,
    Title,
    IsDone,
    CreatedAt,
    FinishedAt,
}
