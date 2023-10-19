use sea_orm_migration::prelude::*;

use super::m20231012_094213_create_user_table::User;
use super::m20231012_094228_create_model_table::Model;
use super::m20231012_122243_create_role_type::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Access::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Access::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Access::Role)
                            .enumeration(Role::Table, [Role::Reader, Role::Commenter, Role::Editor])
                            .not_null(),
                    )
                    .col(ColumnDef::new(Access::ModelId).integer().not_null())
                    .col(ColumnDef::new(Access::UserId).integer().not_null())
                    .index(Index::create().col(Access::ModelId).col(Access::UserId).unique())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Access::Table, Access::ModelId)
                            .to(Model::Table, Model::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Access::Table, Access::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Access::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Access {
    Id,
    Table,
    Role,
    ModelId,
    UserId,
}
