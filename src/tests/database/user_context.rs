use crate::database::database_context;
use crate::database::entity_context;
use crate::database::user_context;
use crate::entities::prelude::User;
use crate::entities::user::{ActiveModel, Model};

#[cfg(test)]
mod database_tests {
    use crate::entities::prelude::User;
    use crate::{
        database::{
            database_context::{DatabaseContext, DatabaseContextTrait},
            entity_context::EntityContextTrait,
            user_context::{self, UserContext},
        },
        entities::user::{self, Entity, Model},
    };
    use sea_orm::{
        entity::prelude::*, entity::*, sea_query::TableCreateStatement, tests_cfg::*, Database,
        DatabaseBackend, DatabaseConnection, MockDatabase, Schema, Transaction,
    };
    use std::any::Any;

    async fn setup_schema(db: &DatabaseConnection) {
        // Setup Schema helper
        let schema = Schema::new(DatabaseBackend::Sqlite);

        // Derive from Entity
        let stmt: TableCreateStatement = schema.create_table_from_entity(Entity);
        let _ = db.execute(db.get_database_backend().build(&stmt)).await;
    }

    /// Sets up a UserContext connected to an in-memory sqlite db
    async fn test_setup() -> UserContext {
        let connection = Database::connect("sqlite::memory:").await.unwrap();
        setup_schema(&connection).await;
        let db_context = DatabaseContext { db: connection };
        UserContext::new(db_context)
    }
    fn two_template_users() -> Vec<Model> {
        vec![
            Model {
                id: 1,
                email: "anders@mail.dk".to_string(),
                username: "anders".to_string(),
                password: "123".to_string(),
            },
            Model {
                id: 2,
                email: "mike@mail.dk".to_string(),
                username: "mikemanden".to_string(),
                password: "qwerty".to_string(),
            },
        ]
    }
    // Test the functionality of the 'create' function, which creates a user in the database
    #[tokio::test]
    async fn create_test() -> Result<(), DbErr> {
        // Setting up a sqlite database in memory to test on
        let db_connection = Database::connect("sqlite::memory:").await.unwrap();
        setup_schema(&db_connection).await;
        let db_context = DatabaseContext { db: db_connection };
        let user_context = UserContext::new(db_context);

        // Creates a model of the user which will be created
        let new_user = Model {
            id: 1,
            email: "anders21@student.aau.dk".to_owned(),
            username: "andemad".to_owned(),
            password: "rask".to_owned(),
        };

        // Creates the user in the database using the 'create' function
        let created_user = user_context.create(new_user).await?;

        let fetched_user = Entity::find_by_id(created_user.id)
            .one(&user_context.db_context.db)
            .await?;

        // Assert if the fetched user is the same as the created user
        assert_eq!(fetched_user.unwrap().username, created_user.username);
        /*
        let db = MockDatabase::new(DatabaseBackend::Postgres).append_query_results([
            vec![user::Model{
                id: 1,
                email: "anders21@student.aau.dk".to_owned(),
                username: "andemad".to_owned(),
                password: "rask".to_owned(),
            }],
            vec![user::Model{
                id: 1,
                email: "anders21@student.aau.dk".to_owned(),
                username: "andemad".to_owned(),
                password: "rask".to_owned(),},
                user::Model{
                    id: 2,
                    email: "andeand@and.and".to_owned(),
                    username: "OgsåAndersRask".to_owned(),
                    password: "rask".to_owned(),
                }
            ]
        ]);
        */

        Ok(())
    }

    #[tokio::test]
    async fn get_by_id_test() -> Result<(), DbErr> {
        // Setting up a sqlite database in memory to test on
        let db_connection = Database::connect("sqlite::memory:").await.unwrap();
        setup_schema(&db_connection).await;
        let db_context = DatabaseContext { db: db_connection };
        let user_context = UserContext::new(db_context);

        // Creates a model of the user which will be created
        let new_user = Model {
            id: 1,
            email: "anders21@student.aau.dk".to_owned(),
            username: "andemad".to_owned(),
            password: "rask".to_owned(),
        };

        // Creates the user in the database using the 'create' function
        let created_user = user_context.create(new_user).await?;

        // Fecthes the user created using the 'get_by_id' function
        let fetched_user = user_context.get_by_id(created_user.id).await;

        // Assert if the fetched user is the same as the created user
        assert_eq!(
            fetched_user.unwrap().unwrap().username,
            created_user.username
        );

        Ok(())
    }
    #[tokio::test]
    async fn get_all_test() -> () {
        let user_context = test_setup().await;

        let mut users_vec: Vec<Model> = vec![
            Model {
                id: 1,
                email: "anders21@student.aau.dk".to_string(),
                username: "anders".to_string(),
                password: "123".to_string(),
            },
            Model {
                id: 2,
                email: "mike@mail.dk".to_string(),
                username: "mikeManden".to_string(),
                password: "qwerty".to_string(),
            },
        ];
        let mut res_users: Vec<Model> = vec![];
        for user in users_vec.iter_mut() {
            res_users.push(user_context.create(user.to_owned()).await.unwrap());
        }
        assert_eq!(users_vec, res_users);
    }

    #[tokio::test]
    async fn update_test() -> () {
        let user_context = test_setup().await;

        let user = Model {
            id: 1,
            email: "anders21@student.aau.dk".to_string(),
            username: "anders".to_string(),
            password: "123".to_string(),
        };
        let user = user_context.create(user).await.unwrap();
        let updated_user = Model {
            password: "qwerty".to_string(),
            ..user
        };
        assert_eq!(
            updated_user,
            user_context.update(updated_user.to_owned()).await.unwrap()
        )
    }

    ///test that where the unique email constraint is violated
    #[tokio::test]
    async fn update_fail() -> () {
        let user_context = test_setup().await;
        let mut users = two_template_users();

        for user in users.iter_mut() {
            let _ = user_context.create(user.to_owned()).await;
        }
        let res = user_context
            .update(Model {
                email: "mike@mail.dk".to_string(),
                ..users[0].to_owned()
            })
            .await;
        match res {
            Ok(_) => {
                panic!("should not happen")
            }
            Err(err) => {
                return;
            }
        }
    }
    #[tokio::test]
    async fn delete_test() -> () {
        let user_context = test_setup().await;
        let mut users = two_template_users();

        for user in users.iter_mut() {
            let _ = user_context.create(user.to_owned()).await;
        }
        assert_eq!(users[0], user_context.delete(users[0].id).await.unwrap())
    }
    #[tokio::test]
    async fn delete_test_fail() -> () {
        let user_context = test_setup().await;
        let mut users = two_template_users();

        for user in users.iter_mut() {
            let _ = user_context.create(user.to_owned()).await;
        }
        let res = user_context.delete(3).await;
        match res {
            Ok(_) => {
                panic!("should not happen")
            }
            Err(err) => {
                return;
            }
        }
    }
}
