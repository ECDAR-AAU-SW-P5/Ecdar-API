use crate::database::database_context::DatabaseContext;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::DbErr;
use std::fmt::Error;

#[async_trait]
pub trait EntityContextTrait<T> {
    fn new(db_context: DatabaseContext) -> Self;
    async fn create(&self, entity: T) -> Result<T, DbErr>;
    async fn get_by_id(&self, entity_id: i32) -> Result<Option<T>, DbErr>;
    async fn get_all(&self) -> Result<Vec<T>, DbErr>;
    async fn update(&self, entity: T) -> Result<T, DbErr>;
    async fn delete(&self, entity_id: i32) -> Result<T, DbErr>;
}