use crate::contexts::context_traits::EntityContextTrait;
use crate::entities::query;
use async_trait::async_trait;
use sea_orm::DbErr;

#[async_trait]
pub trait QueryContextTrait: EntityContextTrait<query::Model> {
    /// Returns the queries associated with a given project id
    async fn get_all_by_project_id(&self, project_id: i32) -> Result<Vec<query::Model>, DbErr>;
}
