use crate::api::server::protobuf::AccessInfo;
use crate::contexts::context_traits::{AccessContextTrait, EntityContextTrait};
use crate::tests::contexts::helpers::{
    create_accesses, create_projects, create_users, get_reset_database_context,
};
use crate::{
    contexts::context_impls::AccessContext,
    entities::{access, project, user},
    to_active_models,
};
use sea_orm::{entity::prelude::*, IntoActiveModel};

async fn seed_db() -> (AccessContext, access::Model, user::Model, project::Model) {
    let db_context = get_reset_database_context().await;

    let access_context = AccessContext::new(db_context);

    let user = create_users(1)[0].clone();
    let project = create_projects(1, user.id)[0].clone();
    let access = create_accesses(1, user.id, project.id)[0].clone();

    user::Entity::insert(user.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();
    project::Entity::insert(project.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    (access_context, access, user, project)
}

// Test the functionality of the 'create' function, which creates a access in the contexts
#[tokio::test]
async fn create_test() {
    let (access_context, access, _, _) = seed_db().await;

    let created_access = access_context.create(access.clone()).await.unwrap();

    let fetched_access = access::Entity::find_by_id(created_access.id)
        .one(&access_context.db_context.get_connection())
        .await
        .unwrap()
        .unwrap();

    // Assert if the fetched access is the same as the created access
    assert_eq!(access, created_access);
    assert_eq!(fetched_access, created_access);
}

#[tokio::test]
async fn create_check_unique_pair_project_id_user_id_test() {
    let (access_context, access, _, _) = seed_db().await;

    let _created_access_1 = access_context.create(access.clone()).await.unwrap();
    let _created_access_2 = access_context.create(access.clone()).await;

    assert!(matches!(
        _created_access_2.unwrap_err().sql_err(),
        Some(SqlErr::UniqueConstraintViolation(_))
    ));
}

#[tokio::test]
async fn create_invalid_role_test() {
    let (access_context, mut access, _, _) = seed_db().await;

    access.role = "abc".into();

    let created_access = access_context.create(access.clone()).await;

    assert!(matches!(
        created_access.unwrap_err().sql_err(),
        Some(SqlErr::ForeignKeyConstraintViolation(_))
    ));
}

#[tokio::test]
async fn create_auto_increment_test() {
    let (access_context, _, user, project_1) = seed_db().await;

    let mut project_2 = create_projects(1, user.id)[0].clone();
    project_2.id = project_1.id + 1;
    project_2.name = "project_2".to_string();

    project::Entity::insert(project_2.into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let access_1 = access::Model {
        id: 0,
        role: "Editor".to_string(),
        project_id: 1,
        user_id: user.id,
    };

    let access_2 = access::Model {
        id: 0,
        role: "Editor".to_string(),
        project_id: 2,
        user_id: user.id,
    };

    let created_access1 = access_context.create(access_1.clone()).await.unwrap();
    let created_access2 = access_context.create(access_2.clone()).await.unwrap();

    let fetched_access1 = access::Entity::find_by_id(created_access1.id)
        .one(&access_context.db_context.get_connection())
        .await
        .unwrap()
        .unwrap();

    let fetched_access2 = access::Entity::find_by_id(created_access2.id)
        .one(&access_context.db_context.get_connection())
        .await
        .unwrap()
        .unwrap();

    assert_ne!(fetched_access1.id, fetched_access2.id);
    assert_ne!(created_access1.id, created_access2.id);
    assert_eq!(created_access1.id, fetched_access1.id);
    assert_eq!(created_access2.id, fetched_access2.id);
}

#[tokio::test]
async fn get_by_id_test() {
    let (access_context, access, _, _) = seed_db().await;

    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    // Fetches the access created using the 'get_by_id' function
    let fetched_access = access_context.get_by_id(access.id).await.unwrap().unwrap();

    // Assert if the fetched access is the same as the created access
    assert_eq!(access, fetched_access);
}

#[tokio::test]
async fn get_by_non_existing_id_test() {
    let (access_context, _, _, _) = seed_db().await;

    let fetched_access = access_context.get_by_id(1).await.unwrap();

    assert!(fetched_access.is_none());
}

#[tokio::test]
async fn get_all_test() {
    let (access_context, _, user, project) = seed_db().await;

    // Creates a model of the access which will be created
    let new_accesses = create_accesses(1, user.id, project.id);

    // Creates the access in the contexts using the 'create' function
    access::Entity::insert_many(to_active_models!(new_accesses.clone()))
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    assert_eq!(access_context.get_all().await.unwrap().len(), 1);

    let mut sorted: Vec<access::Model> = new_accesses.clone();
    sorted.sort_by_key(|k| k.id);

    for (i, access) in sorted.into_iter().enumerate() {
        assert_eq!(access, new_accesses[i]);
    }
}

#[tokio::test]
async fn get_all_empty_test() {
    let (access_context, _, _, _) = seed_db().await;

    let result = access_context.get_all().await.unwrap();
    let empty_accesses: Vec<access::Model> = vec![];

    assert_eq!(empty_accesses, result);
}

#[tokio::test]
async fn update_test() {
    let (access_context, access, _, _) = seed_db().await;

    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let new_access = access::Model { ..access };

    let updated_access = access_context.update(new_access.clone()).await.unwrap();

    let fetched_access = access::Entity::find_by_id(updated_access.id)
        .one(&access_context.db_context.get_connection())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(new_access, updated_access);
    assert_eq!(updated_access, fetched_access);
}

#[tokio::test]
async fn update_modifies_role_test() {
    let (access_context, access, _, _) = seed_db().await;

    let access = access::Model {
        role: "Editor".into(),
        ..access
    };

    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let new_access = access::Model {
        role: "Commenter".into(),
        ..access
    };

    let updated_access = access_context.update(new_access.clone()).await.unwrap();

    assert_ne!(access, updated_access);
    assert_ne!(access, new_access);
}

#[tokio::test]
async fn update_does_not_modify_id_test() {
    let (access_context, access, _, _) = seed_db().await;
    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let updated_access = access::Model {
        id: &access.id + 1,
        ..access.clone()
    };
    let res = access_context.update(updated_access.clone()).await;

    assert!(matches!(res.unwrap_err(), DbErr::RecordNotUpdated));
}

#[tokio::test]
async fn update_does_not_modify_project_id_test() {
    let (access_context, access, _, _) = seed_db().await;

    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let updated_access = access::Model {
        project_id: &access.project_id + 1,
        ..access.clone()
    };
    let res = access_context.update(updated_access.clone()).await.unwrap();

    assert_eq!(access, res);
}

#[tokio::test]
async fn update_does_not_modify_user_id_test() {
    let (access_context, access, _, _) = seed_db().await;

    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let updated_access = access::Model {
        user_id: &access.user_id + 1,
        ..access.clone()
    };
    let res = access_context.update(updated_access.clone()).await.unwrap();

    assert_eq!(access, res);
}

#[tokio::test]
async fn update_invalid_role_test() {
    let (access_context, mut access, _, _) = seed_db().await;

    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    access.role = "abc".into();

    let updated_access = access_context.update(access.clone()).await;

    assert!(matches!(
        updated_access.unwrap_err().sql_err(),
        Some(SqlErr::ForeignKeyConstraintViolation(_))
    ));
}

#[tokio::test]
async fn update_non_existing_id_test() {
    let (access_context, access, _, _) = seed_db().await;

    let updated_access = access_context.update(access.clone()).await;

    assert!(matches!(
        updated_access.unwrap_err(),
        DbErr::RecordNotUpdated
    ));
}

#[tokio::test]
async fn delete_test() {
    let (access_context, access, _, _) = seed_db().await;

    access::Entity::insert(access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let deleted_access = access_context.delete(access.id).await.unwrap();

    let all_accesses = access::Entity::find()
        .all(&access_context.db_context.get_connection())
        .await
        .unwrap();

    assert_eq!(access, deleted_access);
    assert!(all_accesses.is_empty());
}

#[tokio::test]
async fn delete_non_existing_id_test() {
    let (access_context, _, _, _) = seed_db().await;

    let deleted_access = access_context.delete(1).await;

    assert!(matches!(
        deleted_access.unwrap_err(),
        DbErr::RecordNotFound(_)
    ));
}

#[tokio::test]
async fn get_by_uid_and_project_id_test() {
    let (access_context, expected_access, user, project) = seed_db().await;

    access::Entity::insert(expected_access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let access = access_context
        .get_access_by_uid_and_project_id(user.id, project.id)
        .await;

    assert_eq!(access.unwrap().unwrap(), expected_access);
}

#[tokio::test]
async fn get_access_by_project_id_test_returns_ok() {
    let (access_context, expected_access, _, model) = seed_db().await;

    let expected_access_access_info_vector = vec![AccessInfo {
        id: expected_access.id,
        project_id: expected_access.project_id,
        user_id: expected_access.user_id,
        role: expected_access.role.clone(),
    }];

    access::Entity::insert(expected_access.clone().into_active_model())
        .exec(&access_context.db_context.get_connection())
        .await
        .unwrap();

    let access = access_context.get_access_by_project_id(model.id).await;

    assert!(access.unwrap() == expected_access_access_info_vector);
}

#[tokio::test]
async fn get_access_by_project_id_test_returns_empty() {
    let (access_context, _, _, model) = seed_db().await;

    let access = access_context.get_access_by_project_id(model.id).await;

    assert!(access.unwrap().is_empty());
}
