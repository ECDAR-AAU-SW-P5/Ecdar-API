use crate::api::auth::RequestExt;
use crate::api::server::protobuf::{
    CreateQueryRequest, DeleteQueryRequest, QueryRequest, SendQueryRequest, SendQueryResponse,
    UpdateQueryRequest,
};
use crate::contexts::context_collection::ContextCollection;
use crate::controllers::controller_traits::QueryControllerTrait;
use crate::entities::query;
use crate::services::service_collection::ServiceCollection;
use async_trait::async_trait;
use tonic::{Code, Request, Response, Status};

pub struct QueryController {
    contexts: ContextCollection,
    services: ServiceCollection,
}

impl QueryController {
    pub fn new(contexts: ContextCollection, services: ServiceCollection) -> Self {
        Self { contexts, services }
    }
}

#[async_trait]
impl QueryControllerTrait for QueryController {
    async fn create_query(
        &self,
        request: Request<CreateQueryRequest>,
    ) -> Result<Response<()>, Status> {
        let query_request = request.get_ref();

        let access = self
            .contexts
            .access_context
            .get_access_by_uid_and_project_id(
                request
                    .uid()
                    .map_err(|err| {
                        Status::invalid_argument(format!(
                            "could not stringify user id in request metadata, inner error {}",
                            err
                        ))
                    })?
                    .ok_or(Status::invalid_argument(
                        "failed to get user id from request metadata",
                    ))?,
                query_request.project_id,
            )
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?
            .ok_or_else(|| {
                Status::new(
                    Code::PermissionDenied,
                    "User does not have access to project",
                )
            })?;

        if access.role != "Editor" {
            return Err(Status::new(
                Code::PermissionDenied,
                "Role does not have permission to create query",
            ));
        }

        let query = query::Model {
            id: Default::default(),
            string: query_request.string.to_string(),
            result: Default::default(),
            outdated: Default::default(),
            project_id: query_request.project_id,
        };

        match self.contexts.query_context.create(query).await {
            Ok(_) => Ok(Response::new(())),
            Err(error) => Err(Status::new(Code::Internal, error.to_string())),
        }
    }

    async fn update_query(
        &self,
        request: Request<UpdateQueryRequest>,
    ) -> Result<Response<()>, Status> {
        let message = request.get_ref().clone();

        let old_query_res = self
            .contexts
            .query_context
            .get_by_id(message.id)
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?;

        let old_query = match old_query_res {
            Some(oq) => oq,
            None => return Err(Status::new(Code::NotFound, "Query not found".to_string())),
        };

        let access = self
            .contexts
            .access_context
            .get_access_by_uid_and_project_id(
                request
                    .uid()
                    .map_err(|err| {
                        Status::internal(format!(
                            "could not stringify user id in request metadata, internal error {}",
                            err
                        ))
                    })?
                    .ok_or(Status::internal(
                        "failed to get user id from request metadata",
                    ))?,
                old_query.project_id,
            )
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?
            .ok_or_else(|| {
                Status::new(
                    Code::PermissionDenied,
                    "User does not have access to project",
                )
            })?;

        if access.role != "Editor" {
            return Err(Status::new(
                Code::PermissionDenied,
                "Role does not have permission to update query",
            ));
        }

        let query = query::Model {
            id: message.id,
            project_id: Default::default(),
            string: message.string,
            result: old_query.result,
            outdated: old_query.outdated,
        };

        match self.contexts.query_context.update(query).await {
            Ok(_) => Ok(Response::new(())),
            Err(error) => Err(Status::new(Code::Internal, error.to_string())),
        }
    }

    async fn delete_query(
        &self,
        request: Request<DeleteQueryRequest>,
    ) -> Result<Response<()>, Status> {
        let message = request.get_ref();

        let query = self
            .contexts
            .query_context
            .get_by_id(message.id)
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?
            .ok_or_else(|| Status::new(Code::NotFound, "Query not found"))?;

        let access = self
            .contexts
            .access_context
            .get_access_by_uid_and_project_id(
                request
                    .uid()
                    .map_err(|err| {
                        Status::internal(format!(
                            "could not stringify user id in request metadata, internal error {}",
                            err
                        ))
                    })?
                    .ok_or(Status::internal(
                        "failed to get user id from request metadata",
                    ))?,
                query.project_id,
            )
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?
            .ok_or_else(|| {
                Status::new(
                    Code::PermissionDenied,
                    "User does not have access to project",
                )
            })?;

        if access.role != "Editor" {
            return Err(Status::new(
                Code::PermissionDenied,
                "Role does not have permission to update query",
            ));
        }

        match self.contexts.query_context.delete(message.id).await {
            Ok(_) => Ok(Response::new(())),
            Err(error) => match error {
                sea_orm::DbErr::RecordNotFound(message) => {
                    Err(Status::new(Code::NotFound, message))
                }
                _ => Err(Status::new(Code::Internal, error.to_string())),
            },
        }
    }

    async fn send_query(
        &self,
        request: Request<SendQueryRequest>,
    ) -> Result<Response<SendQueryResponse>, Status> {
        let message = request.get_ref();

        let uid = request
            .uid()
            .map_err(|err| {
                Status::internal(format!(
                    "could not stringify user id in request metadata, internal error {}",
                    err
                ))
            })?
            .ok_or(Status::internal(
                "failed to get user id from request metadata",
            ))?;

        // Verify user access
        self.contexts
            .access_context
            .get_access_by_uid_and_project_id(uid, message.project_id)
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?
            .ok_or_else(|| {
                Status::new(
                    Code::PermissionDenied,
                    "User does not have access to project",
                )
            })?;

        // Get project from contexts
        let project = self
            .contexts
            .project_context
            .get_by_id(message.project_id)
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?
            .ok_or_else(|| Status::new(Code::NotFound, "Model not found"))?;

        // Get query from contexts
        let query = self
            .contexts
            .query_context
            .get_by_id(message.id)
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?
            .ok_or_else(|| Status::new(Code::NotFound, "Query not found"))?;

        // Construct query request to send to Reveaal
        let query_request = Request::new(QueryRequest {
            user_id: uid,
            query_id: message.id,
            query: query.string.clone(),
            components_info: serde_json::from_value(project.components_info).map_err(|err| {
                Status::internal(format!(
                    "error parsing query result, internal error: {}",
                    err
                ))
            })?,
            settings: Default::default(), //TODO
        });

        // Run query on Reveaal
        let query_result = self
            .services
            .reveaal_service
            .send_query(query_request)
            .await?;

        // Update query result in contexts
        self.contexts
            .query_context
            .update(query::Model {
                id: query.id,
                string: query.string.clone(),
                result: Some(
                    serde_json::to_value(
                        query_result
                            .get_ref()
                            .result
                            .clone()
                            .ok_or(Status::internal("failed to get query result"))?, //TODO better error message ?
                    )
                    .map_err(|err| {
                        Status::internal(format!(
                            "error parsing query result, internal error: {}",
                            err
                        ))
                    })?,
                ),
                outdated: false,
                project_id: query.project_id,
            })
            .await
            .map_err(|err| Status::new(Code::Internal, err.to_string()))?;

        Ok(Response::new(SendQueryResponse {
            response: Some(query_result.into_inner()),
        }))
    }
}

#[cfg(test)]
#[path = "../../tests/controllers/query_controller.rs"]
mod query_controller_tests;
