#[cfg(test)]
mod query_logic {
    use crate::api::server::server::ecdar_api_server::EcdarApi;
    use crate::api::server::server::{CreateQueryRequest, DeleteQueryRequest, UpdateQueryRequest};
    use crate::entities::query;
    use crate::tests::api::helpers::{get_mock_concrete_ecdar_api, get_mock_services};
    use mockall::predicate;
    use sea_orm::DbErr;
    use tonic::{Code, Request};

    #[tokio::test]
    async fn create_invalid_query_returns_err() {
        let mut mock_services = get_mock_services();

        let query = query::Model {
            id: Default::default(),
            string: "".to_string(),
            result: Default::default(),
            model_id: 1,
            outdated: Default::default(),
        };

        mock_services
            .query_context_mock
            .expect_create()
            .with(predicate::eq(query.clone()))
            .returning(move |_| Err(DbErr::RecordNotInserted));

        let request = Request::new(CreateQueryRequest {
            string: "".to_string(),
            model_id: 1,
        });

        let api = get_mock_concrete_ecdar_api(mock_services);

        let res = api.create_query(request).await.unwrap_err();

        assert_eq!(res.code(), Code::Internal);
    }

    #[tokio::test]
    async fn create_query_returns_ok() {
        let mut mock_services = get_mock_services();

        let query = query::Model {
            id: Default::default(),
            string: "".to_string(),
            result: Default::default(),
            model_id: 1,
            outdated: Default::default(),
        };

        mock_services
            .query_context_mock
            .expect_create()
            .with(predicate::eq(query.clone()))
            .returning(move |_| Ok(query.clone()));

        let request = Request::new(CreateQueryRequest {
            string: "".to_string(),
            model_id: 1,
        });

        let api = get_mock_concrete_ecdar_api(mock_services);

        let res = api.create_query(request).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn update_invalid_query_returns_err() {
        let mut mock_services = get_mock_services();

        let old_query = query::Model {
            id: 1,
            string: "".to_string(),
            result: None,
            model_id: Default::default(),
            outdated: true,
        };

        let query = query::Model {
            string: "updated".to_string(),
            ..old_query.clone()
        };

        mock_services
            .query_context_mock
            .expect_get_by_id()
            .with(predicate::eq(1))
            .returning(move |_| Ok(Some(old_query.clone())));

        mock_services
            .query_context_mock
            .expect_update()
            .with(predicate::eq(query.clone()))
            .returning(move |_| Err(DbErr::RecordNotUpdated));

        let request = Request::new(UpdateQueryRequest {
            id: 1,
            string: "updated".to_string(),
        });

        let api = get_mock_concrete_ecdar_api(mock_services);

        let res = api.update_query(request).await.unwrap_err();

        assert_eq!(res.code(), Code::Internal);
    }

    #[tokio::test]
    async fn update_query_returns_ok() {
        let mut mock_services = get_mock_services();

        let old_query = query::Model {
            id: 1,
            string: "".to_string(),
            result: None,
            model_id: Default::default(),
            outdated: true,
        };

        let query = query::Model {
            string: "updated".to_string(),
            ..old_query.clone()
        };

        mock_services
            .query_context_mock
            .expect_get_by_id()
            .with(predicate::eq(1))
            .returning(move |_| Ok(Some(old_query.clone())));

        mock_services
            .query_context_mock
            .expect_update()
            .with(predicate::eq(query.clone()))
            .returning(move |_| Ok(query.clone()));

        let request = Request::new(UpdateQueryRequest {
            id: 1,
            string: "updated".to_string(),
        });

        let api = get_mock_concrete_ecdar_api(mock_services);

        let res = api.update_query(request).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn delete_invalid_query_returns_err() {
        let mut mock_services = get_mock_services();

        mock_services
            .query_context_mock
            .expect_delete()
            .with(predicate::eq(1))
            .returning(move |_| Err(DbErr::RecordNotFound("".to_string())));

        let request = Request::new(DeleteQueryRequest { id: 1 });

        let api = get_mock_concrete_ecdar_api(mock_services);

        let res = api.delete_query(request).await.unwrap_err();

        assert_eq!(res.code(), Code::NotFound);
    }

    #[tokio::test]
    async fn delete_query_returns_ok() {
        let mut mock_services = get_mock_services();

        let query = query::Model {
            id: 1,
            string: "".to_string(),
            result: Default::default(),
            model_id: Default::default(),
            outdated: Default::default(),
        };

        mock_services
            .query_context_mock
            .expect_delete()
            .with(predicate::eq(1))
            .returning(move |_| Ok(query.clone()));

        let request = Request::new(DeleteQueryRequest { id: 1 });

        let api = get_mock_concrete_ecdar_api(mock_services);

        let res = api.delete_query(request).await;

        assert!(res.is_ok());
    }
}