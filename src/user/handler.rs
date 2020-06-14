use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::errors::ApiError;
use super::model::{User, NewUser, UpdateUser};

pub async fn get_users() -> Result<HttpResponse, ApiError> {   
    let users = User::get()?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(data: web::Json<NewUser>) -> Result<HttpResponse, ApiError> {
    let user = User::create(data.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn get_user_by_id(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(data: web::Json<UpdateUser>) -> Result<HttpResponse, ApiError> {
    let user = User::update(data.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let count = User::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": count})))
}

