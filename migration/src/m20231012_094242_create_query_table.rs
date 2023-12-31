use sea_orm_migration::prelude::*;

use super::m20231012_094228_create_project_table::Project;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Query::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Query::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Query::String).string().not_null())
                    .col(ColumnDef::new(Query::Result).json())
                    .col(
                        ColumnDef::new(Query::Outdated)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Query::ProjectId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Query::Table, Query::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Query::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Query {
    Table,
    Id,
    String,
    Result,
    ProjectId,
    Outdated,
}
