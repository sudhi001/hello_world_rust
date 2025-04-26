use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use uuid::Uuid;
use log::error;
use std::error::Error as StdError;

use crate::models::user::{CreateUserRequest, UpdateUserRequest};
use crate::repositories::user_repo::UserRepository;

// GET /health - Health check endpoint
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

// GET /users - List all users
#[get("/users")]
pub async fn get_users(repo: web::Data<UserRepository>) -> impl Responder {
    match repo.get_all().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            error!("Failed to get users: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve users"
            }))
        }
    }
}

// GET /users/{id} - Get a specific user
#[get("/users/{id}")]
pub async fn get_user(path: web::Path<Uuid>, repo: web::Data<UserRepository>) -> impl Responder {
    let user_id = path.into_inner();
    
    match repo.get_by_id(&user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => {
            error!("Failed to get user {}: {}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve user"
            }))
        }
    }
}

// POST /users - Create a new user
#[post("/users")]
pub async fn create_user(user_req: web::Json<CreateUserRequest>, repo: web::Data<UserRepository>) -> impl Responder {
    match repo.create(&user_req).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
}

// PUT /users/{id} - Update a user
#[put("/users/{id}")]
pub async fn update_user(
    path: web::Path<Uuid>,
    user_req: web::Json<UpdateUserRequest>,
    repo: web::Data<UserRepository>
) -> impl Responder {
    let user_id = path.into_inner();
    
    match repo.update(&user_id, &user_req).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => {
            error!("Failed to update user {}: {}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update user"
            }))
        }
    }
}

// DELETE /users/{id} - Delete a user
#[delete("/users/{id}")]
pub async fn delete_user(path: web::Path<Uuid>, repo: web::Data<UserRepository>) -> impl Responder {
    let user_id = path.into_inner();
    
    match repo.delete(&user_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => {
            error!("Failed to delete user {}: {}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete user"
            }))
        }
    }
}