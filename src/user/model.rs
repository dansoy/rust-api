use diesel::{QueryDsl, RunQueryDsl, insert_into, delete};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use argon2::Config;
use rand::Rng;
use crate::db;
use crate::errors::ApiError;
use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Users(pub Vec<User>);

#[derive(Deserialize)]
pub struct NewUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name="users"]
pub struct UpdateUser {
    pub id: String,
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
 }

impl User {
    pub fn get() -> Result<Users, ApiError> {
        let connection = db::connection()?;
        
        let result = users::table
            //.limit(10)
            .load::<User>(&connection)?;

        Ok(Users(result))
    }

    pub fn create(data: NewUser) -> Result<User, ApiError> {
        let connection = db::connection()?;

        let mut new_user = User::from(data);
        new_user.hash_password()?;
        insert_into(users::table)
                .values(&new_user)
                .execute(&connection)?;

        let result = users::table
                    .find(new_user.id)
                    .first(&connection)?;

        Ok(result)
    }

    pub fn find(id: String) -> Result<User, ApiError> {
        let connection = db::connection()?;

        let result = users::table
            .find(id)
            .first(&connection)?;

        Ok(result)
    }

    pub fn update(mut data: UpdateUser) -> Result<User, ApiError> {
        let connection = db::connection()?;

        data.updated_at = Some(Utc::now().naive_utc());
        
        diesel::update(users::table.find(&data.id))
        .set(&data)
        .execute(&connection)?;

        let result = users::table
            .find(data.id)
            .first(&connection)?;

        Ok(result)
    }

    pub fn delete(id: String) -> Result<usize, ApiError> {
        let connection = db::connection()?;

        let count = delete(users::table.find(id)).execute(&connection)?;
        Ok(count)
    }

    pub fn find_by_email(email: String) -> Result<User, ApiError> {
        let connection = db::connection()?;

        let user = users::table
            .filter(users::email.eq(email))
            .first(&connection)?;

        Ok(user)
    }

    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    }    
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            user_name: user.user_name,
            email: user.email,
            password: user.password,
            first_name: None,
            last_name: None,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}