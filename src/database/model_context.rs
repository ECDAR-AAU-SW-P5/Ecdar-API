use crate::database::database_context::DatabaseContextTrait;
use crate::entities::{access, model, query};

use crate::api::server::server::ModelInfo;
use crate::EntityContextTrait;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbBackend, DbErr, EntityTrait, IntoActiveModel, JoinType,
    ModelTrait, QueryFilter, QuerySelect, QueryTrait, RelationTrait, Set, Unchanged,
};
use std::sync::Arc;

use super::database_context;

pub struct ModelContext {
    db_context: Arc<dyn DatabaseContextTrait>,
}

#[async_trait]
pub trait ModelContextTrait: EntityContextTrait<model::Model> {
    async fn get_models_info_by_uid(&self, uid: i32) -> Result<Vec<ModelInfo>, DbErr>;
}

#[async_trait]
impl ModelContextTrait for ModelContext {
    async fn get_models_info_by_uid(&self, uid: i32) -> Result<Vec<ModelInfo>, DbErr> {
        /*let querytest = access::Entity::find()
        .select_only()
        .column_as(model::Column::Id, "model_id")
        .column_as(model::Column::Name, "model_name")
        .column_as(model::Column::OwnerId, "model_owner_id")
        .column_as(access::Column::Role, "user_role_on_model")
        .join(JoinType::InnerJoin, access::Relation::Model.def())
        .join(JoinType::InnerJoin, access::Relation::Role.def())
        .group_by(model::Column::Id)
        .group_by(access::Column::Role)
        .filter(access::Column::UserId.eq(uid))
        .build(DbBackend::Postgres)
        .to_string();

        println!("SQL Query: {}", querytest);*/

        //join model, access and role tables
        access::Entity::find()
            .select_only()
            .column_as(model::Column::Id, "model_id")
            .column_as(model::Column::Name, "model_name")
            .column_as(model::Column::OwnerId, "model_owner_id")
            .column_as(access::Column::Role, "user_role_on_model")
            .join(JoinType::InnerJoin, access::Relation::Model.def())
            .join(JoinType::InnerJoin, access::Relation::Role.def())
            .group_by(model::Column::Id)
            .group_by(access::Column::Role)
            .filter(access::Column::UserId.eq(uid))
            .into_model::<ModelInfo>()
            .all(&self.db_context.get_connection())
            .await
    }
}

impl ModelContext {
    pub fn new(db_context: Arc<dyn DatabaseContextTrait>) -> ModelContext {
        ModelContext { db_context }
    }
}
#[async_trait]
impl EntityContextTrait<model::Model> for ModelContext {
    /// Used for creating a model::Model entity
    /// # Example
    /// ```
    /// let model = model::Model {
    ///     id: Default::default(),
    ///     name: "model::Model name".to_owned(),
    ///     components_info: "{}".to_owned().parse().unwrap(),
    ///     owner_id: 1
    /// };
    /// let model_context: ModelContext = ModelContext::new(...);
    /// model_context.create(model);
    /// ```
    async fn create(&self, entity: model::Model) -> Result<model::Model, DbErr> {
        let model = model::ActiveModel {
            id: Default::default(),
            name: Set(entity.name),
            components_info: Set(entity.components_info),
            owner_id: Set(entity.owner_id),
        };
        let model: model::Model = model.insert(&self.db_context.get_connection()).await?;
        Ok(model)
    }

    /// Returns a single model entity (Uses primary key)
    /// # Example
    /// ```
    /// let model_context: ModelContext = ModelContext::new(...);
    /// let model = model_context.get_by_id(1).unwrap();
    /// ```
    async fn get_by_id(&self, entity_id: i32) -> Result<Option<model::Model>, DbErr> {
        model::Entity::find_by_id(entity_id)
            .one(&self.db_context.get_connection())
            .await
    }

    /// Returns a all model entities (Uses primary key)
    /// # Example
    /// ```
    /// let model_context: ModelContext = ModelContext::new(...);
    /// let model = model_context.get_all().unwrap();
    /// ```
    async fn get_all(&self) -> Result<Vec<model::Model>, DbErr> {
        model::Entity::find()
            .all(&self.db_context.get_connection())
            .await
    }

    /// Updates a single model entity
    /// # Example
    /// ```
    /// let update_model = model::Model {
    ///     name: "new name",
    ///     ..original_model
    /// };
    ///
    /// let model_context: ModelContext = ModelContext::new(...);
    /// let model = model_context.update(update_model).unwrap();
    /// ```
    async fn update(&self, entity: model::Model) -> Result<model::Model, DbErr> {
        let existing_model = self.get_by_id(entity.id).await?;

        return match existing_model {
            None => Err(DbErr::RecordNotUpdated),
            Some(existing_model) => {
                let queries: Vec<query::Model> = existing_model
                    .find_related(query::Entity)
                    .all(&self.db_context.get_connection())
                    .await?;
                for q in queries.iter() {
                    let mut aq = q.clone().into_active_model();
                    aq.outdated = Set(true);
                    aq.update(&self.db_context.get_connection()).await?;
                }
                model::ActiveModel {
                    id: Unchanged(entity.id),
                    name: Set(entity.name),
                    components_info: Set(entity.components_info),
                    owner_id: Unchanged(entity.id),
                }
                .update(&self.db_context.get_connection())
                .await
            }
        };
    }

    /// Returns and deletes a single model entity
    /// # Example
    /// ```
    /// let model_context: ModelContext = ModelContext::new(...);
    /// let model = model_context.delete().unwrap();
    /// ```
    async fn delete(&self, entity_id: i32) -> Result<model::Model, DbErr> {
        let model = self.get_by_id(entity_id).await?;
        match model {
            None => Err(DbErr::RecordNotFound("No record was deleted".into())),
            Some(model) => {
                model::Entity::delete_by_id(entity_id)
                    .exec(&self.db_context.get_connection())
                    .await?;
                Ok(model)
            }
        }
    }
}

#[cfg(test)]
#[path = "../tests/database/model_context.rs"]
mod model_context_tests;
