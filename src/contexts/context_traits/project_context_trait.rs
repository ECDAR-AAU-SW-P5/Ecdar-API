use crate::api::server::protobuf::ProjectInfo;
use crate::contexts::context_traits::EntityContextTrait;
use crate::entities::project;
use async_trait::async_trait;
use sea_orm::DbErr;

#[async_trait]
pub trait ProjectContextTrait: EntityContextTrait<project::Model> {
    /// Returns the projects owned by a given user id
    /// # Errors
    /// Errors on failed connection, execution error or constraint violations.
    async fn get_project_info_by_uid(&self, uid: i32) -> Result<Vec<ProjectInfo>, DbErr>;
}
