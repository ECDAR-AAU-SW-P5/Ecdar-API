//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub components_info: Json,
    pub owner_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::access::Entity")]
    Access,
    #[sea_orm(has_many = "super::in_use::Entity")]
    InUse,
    #[sea_orm(has_many = "super::query::Entity")]
    Query,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::access::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Access.def()
    }
}

impl Related<super::in_use::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InUse.def()
    }
}

impl Related<super::query::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Query.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
