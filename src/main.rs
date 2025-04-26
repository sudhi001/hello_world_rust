mod config;
mod models;
mod repositories;
mod routes;

use std::process;
use actix_web::{web, App, HttpServer, middleware::Logger};
use config::AppConfig;
use repositories::user_repo::UserRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Load configuration from environment
    let config = match AppConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            log::error!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };
    
    // Create user repository
    let user_repository = UserRepository::new(config.pg_pool.clone());
    
    // Initialize database schema
    match user_repository.init_db().await {
        Ok(_) => log::info!("Database schema initialized successfully"),
        Err(e) => {
            eprintln!("Failed to initialize database schema: {}", e);
            log::error!("Failed to initialize database schema: {}", e);
            process::exit(1);
        }
    }
    
    // Seed sample data
    match user_repository.seed_sample_data().await {
        Ok(_) => log::info!("Sample data seeded successfully"),
        Err(e) => {
            log::warn!("Failed to seed sample data: {}", e);
            // Don't exit on seeding failure, it's not critical
        }
    }
    
    let user_repo_data = web::Data::new(user_repository);
    
    log::info!("Starting server at http://{}:{}", config.host, config.port);
    
    // Start HTTP server
    HttpServer::new(move || {
        let user_repo = user_repo_data.clone();
        App::new()
            .wrap(Logger::default())
            .app_data(user_repo)
            .service(routes::user::health_check)
            .service(routes::user::get_users)
            .service(routes::user::get_user)
            .service(routes::user::create_user)
            .service(routes::user::update_user)
            .service(routes::user::delete_user)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}