use deadpool_postgres::Pool;
use uuid::Uuid;
use std::error::Error as StdError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::models::user::{User, CreateUserRequest, UpdateUserRequest};

// Original repository for database operations
pub struct UserRepository {
    pool: Pool,
}

// New cached repository that wraps the original
pub struct CachedUserRepository {
    repo: UserRepository,
    cache: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn init_db(&self) -> Result<(), Box<dyn StdError>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to get DB client: {}", e);
                return Err(Box::new(e));
            }
        };
        
        // Create users table if it doesn't exist
        client
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id UUID PRIMARY KEY,
                    name VARCHAR(100) NOT NULL,
                    email VARCHAR(255) NOT NULL UNIQUE,
                    age SMALLINT
                )",
                &[],
            )
            .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<User>, Box<dyn StdError>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to get DB client: {}", e);
                return Err(Box::new(e));
            }
        };
        
        let rows = client
            .query("SELECT id, name, email, age FROM users", &[])
            .await?;

        Ok(rows
            .iter()
            .map(|row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                age: row.get::<_, Option<i16>>(3).map(|age| age as u8),
            })
            .collect())
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn StdError>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to get DB client: {}", e);
                return Err(Box::new(e));
            }
        };
        
        let row = client
            .query_opt(
                "SELECT id, name, email, age FROM users WHERE id = $1",
                &[id],
            )
            .await?;

        Ok(row.map(|row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            age: row.get::<_, Option<i16>>(3).map(|age| age as u8),
        }))
    }

    pub async fn create(&self, user_req: &CreateUserRequest) -> Result<User, Box<dyn StdError>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to get DB client: {}", e);
                return Err(Box::new(e));
            }
        };
        
        let user_id = Uuid::new_v4();
        let age: Option<i16> = user_req.age.map(|a| a as i16);
        
        client
            .execute(
                "INSERT INTO users (id, name, email, age) VALUES ($1, $2, $3, $4)",
                &[&user_id, &user_req.name, &user_req.email, &age],
            )
            .await?;

        Ok(User {
            id: user_id,
            name: user_req.name.clone(),
            email: user_req.email.clone(),
            age: user_req.age,
        })
    }

    pub async fn update(&self, id: &Uuid, user_req: &UpdateUserRequest) -> Result<Option<User>, Box<dyn StdError>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to get DB client: {}", e);
                return Err(Box::new(e));
            }
        };
        
        // First check if the user exists
        let existing_user = self.get_by_id(id).await?;
        if existing_user.is_none() {
            return Ok(None);
        }

        let existing_user = existing_user.unwrap();
        
        // Build update query dynamically based on provided fields
        let mut query_parts = Vec::new();
        let mut param_values: Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> = Vec::new();
        
        let mut param_idx = 1;
        
        if let Some(name) = &user_req.name {
            query_parts.push(format!("name = ${}", param_idx));
            param_values.push(Box::new(name.clone()));
            param_idx += 1;
        }
        
        if let Some(email) = &user_req.email {
            query_parts.push(format!("email = ${}", param_idx));
            param_values.push(Box::new(email.clone()));
            param_idx += 1;
        }
        
        if user_req.age.is_some() {
            query_parts.push(format!("age = ${}", param_idx));
            let age: Option<i16> = user_req.age.map(|a| a as i16);
            param_values.push(Box::new(age));
            param_idx += 1;
        }
        
        if query_parts.is_empty() {
            // Nothing to update
            return Ok(Some(existing_user));
        }
        
        // Build the full query
        let query = format!(
            "UPDATE users SET {} WHERE id = ${}",
            query_parts.join(", "),
            param_idx
        );
        
        // Add the id as the last parameter
        param_values.push(Box::new(*id));
        
        // Convert param_values to a slice of &(dyn ToSql + Sync)
        let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = param_values
            .iter()
            .map(|p| p.as_ref())
            .collect();

        // Execute the query
        let rows_affected = client.execute(&query, &params[..]).await?;
        
        if rows_affected == 0 {
            return Ok(None);
        }
        
        // Construct the updated user
        let updated_user = User {
            id: existing_user.id,
            name: user_req.name.clone().unwrap_or(existing_user.name),
            email: user_req.email.clone().unwrap_or(existing_user.email),
            age: user_req.age.or(existing_user.age),
        };
        
        Ok(Some(updated_user))
    }

    pub async fn delete(&self, id: &Uuid) -> Result<bool, Box<dyn StdError>> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to get DB client: {}", e);
                return Err(Box::new(e));
            }
        };
        
        let rows_affected = client
            .execute("DELETE FROM users WHERE id = $1", &[id])
            .await?;
            
        Ok(rows_affected > 0)
    }

    pub async fn seed_sample_data(&self) -> Result<(), Box<dyn StdError>> {
        // Check if we already have users
        let users = self.get_all().await?;
        if !users.is_empty() {
            return Ok(());
        }
        
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to get DB client: {}", e);
                return Err(Box::new(e));
            }
        };
        
        let sample_id = Uuid::new_v4();
        let age: Option<i16> = Some(30);
        
        client
            .execute(
                "INSERT INTO users (id, name, email, age) VALUES ($1, $2, $3, $4)",
                &[
                    &sample_id,
                    &"John Doe".to_string(),
                    &"john@example.com".to_string(),
                    &age,
                ],
            )
            .await?;
            
        Ok(())
    }
}

impl CachedUserRepository {
    pub fn new(pool: Pool) -> Self {
        Self {
            repo: UserRepository::new(pool),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn init_db(&self) -> Result<(), Box<dyn StdError>> {
        self.repo.init_db().await
    }

    pub async fn get_all(&self) -> Result<Vec<User>, Box<dyn StdError>> {
        // Read from DB first
        let users = self.repo.get_all().await?;
        
        // Update cache with all users
        {
            let mut cache = self.cache.write().unwrap();
            for user in &users {
                cache.insert(user.id, user.clone());
            }
        }
        
        Ok(users)
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn StdError>> {
        // Check cache first
        {
            let cache = self.cache.read().unwrap();
            if let Some(user) = cache.get(id) {
                log::debug!("Cache hit for user with id: {}", id);
                return Ok(Some(user.clone()));
            }
        }
        
        // If not in cache, get from DB
        log::debug!("Cache miss for user with id: {}", id);
        let user_option = self.repo.get_by_id(id).await?;
        
        // If found, update cache
        if let Some(ref user) = user_option {
            let mut cache = self.cache.write().unwrap();
            cache.insert(user.id, user.clone());
        }
        
        Ok(user_option)
    }

    pub async fn create(&self, user_req: &CreateUserRequest) -> Result<User, Box<dyn StdError>> {
        // Create in DB first
        let user = self.repo.create(user_req).await?;
        
        // Then update cache
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(user.id, user.clone());
        }
        
        Ok(user)
    }

    pub async fn update(&self, id: &Uuid, user_req: &UpdateUserRequest) -> Result<Option<User>, Box<dyn StdError>> {
        // Update in DB first
        let updated_user = self.repo.update(id, user_req).await?;
        
        // Then update cache if user exists
        if let Some(ref user) = updated_user {
            let mut cache = self.cache.write().unwrap();
            cache.insert(user.id, user.clone());
        } else {
            // If user doesn't exist anymore, remove from cache
            let mut cache = self.cache.write().unwrap();
            cache.remove(id);
        }
        
        Ok(updated_user)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<bool, Box<dyn StdError>> {
        // Delete from DB first
        let deleted = self.repo.delete(id).await?;
        
        // If deleted, remove from cache
        if deleted {
            let mut cache = self.cache.write().unwrap();
            cache.remove(id);
        }
        
        Ok(deleted)
    }

    pub async fn seed_sample_data(&self) -> Result<(), Box<dyn StdError>> {
        // Seed data in DB
        let result = self.repo.seed_sample_data().await?;
        
        // Then refresh cache with all users
        let _ = self.get_all().await?;
        
        Ok(result)
    }
    
    // Method to manually invalidate cache for testing or administrative purposes
    pub fn invalidate_cache(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
        log::info!("User cache invalidated");
    }
    
    // Method to refresh single cache entry
    pub async fn refresh_cache_entry(&self, id: &Uuid) -> Result<(), Box<dyn StdError>> {
        let user_option = self.repo.get_by_id(id).await?;
        
        let mut cache = self.cache.write().unwrap();
        if let Some(user) = user_option {
            cache.insert(user.id, user.clone());
        } else {
            cache.remove(id);
        }
        
        Ok(())
    }
}