use deadpool_postgres::{Config as PgConfig, Pool, Runtime, SslMode};
use dotenv::dotenv;
use std::env;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;

pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub pg_pool: Pool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Load environment variables from .env file
        dotenv().ok();

        // Server config
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;

        // Create PostgreSQL configuration
        let pg_config = match env::var("DATABASE_URL") {
            Ok(url) => {
                // Parse connection string manually
                log::info!("Using DATABASE_URL from environment");
                let mut config = PgConfig::new();
                
                if let Err(e) = Self::parse_db_url(&url, &mut config) {
                    log::warn!("Failed to parse DATABASE_URL: {}", e);
                    Self::default_db_config()
                } else {
                    // Verify the config has required fields
                    if config.dbname.is_none() || config.dbname.as_ref().map_or(true, |s| s.is_empty()) {
                        log::warn!("Database name is empty in DATABASE_URL, using default");
                        config.dbname = Some("postgres".to_string());
                    }
                    
                    if config.user.is_none() {
                        log::warn!("User is not specified in DATABASE_URL, using default");
                        config.user = Some("postgres".to_string());
                    }
                    
                    if config.host.is_none() {
                        log::warn!("Host is not specified in DATABASE_URL, using default");
                        config.host = Some("localhost".to_string());
                    }
                    
                    config
                }
            },
            Err(_) => {
                // Use individual parameters
                log::info!("DATABASE_URL not found, using individual parameters");
                let mut config = PgConfig::new();
                
                config.host = Some(env::var("PG_HOST").unwrap_or_else(|_| "localhost".to_string()));
                config.port = Some(
                    env::var("PG_PORT")
                        .unwrap_or_else(|_| "5432".to_string())
                        .parse::<u16>()
                        .unwrap_or(5432),
                );
                config.dbname = Some(env::var("PG_DBNAME").unwrap_or_else(|_| "postgres".to_string()));
                config.user = Some(env::var("PG_USER").unwrap_or_else(|_| "postgres".to_string()));
                config.password = Some(env::var("PG_PASSWORD").unwrap_or_else(|_| "postgres".to_string()));
                
                // Check for SSL mode
                if let Ok(ssl_mode) = env::var("PG_SSLMODE") {
                    if ssl_mode == "require" {
                        config.ssl_mode = Some(SslMode::Require);
                    }
                }
                
                config
            }
        };
        
        // Log configuration for debugging
        log::info!("PostgreSQL Configuration:");
        log::info!("  Host: {}", pg_config.host.as_deref().unwrap_or("not set"));
        log::info!("  Port: {}", pg_config.port.unwrap_or(5432));
        log::info!("  Database: {}", pg_config.dbname.as_deref().unwrap_or("not set"));
        log::info!("  User: {}", pg_config.user.as_deref().unwrap_or("not set"));
        log::info!("  SSL Mode: {}", pg_config.ssl_mode.as_ref().map_or("not set", |m| match m {
            SslMode::Disable => "disable",
            SslMode::Prefer => "prefer",
            SslMode::Require => "require",
            _ => "other"
        }));
        
        // Create the connection pool with TLS if required
        let pg_pool = if pg_config.ssl_mode.as_ref().map_or(false, |m| *m == SslMode::Require) {
            log::info!("Using TLS for PostgreSQL connection");
            // Use TLS connector for secure connections
            let tls_connector = TlsConnector::builder()
                .danger_accept_invalid_certs(true) // For self-signed certificates
                .build()?;
            let connector = MakeTlsConnector::new(tls_connector);
            pg_config.create_pool(Some(Runtime::Tokio1), connector)?
        } else {
            log::info!("Using no TLS for PostgreSQL connection");
            // For local development without TLS
            let connector = postgres_native_tls::MakeTlsConnector::new(
                TlsConnector::builder()
                    .danger_accept_invalid_certs(true)
                    .build()?
            );
            pg_config.create_pool(Some(Runtime::Tokio1), connector)?
        };
        
        log::info!("PostgreSQL connection pool created successfully");

        Ok(Self {
            host,
            port,
            pg_pool,
        })
    }
    
    fn parse_db_url(url: &str, config: &mut PgConfig) -> Result<(), String> {
        // Accept both postgres:// and postgresql:// protocol prefixes
        if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
            return Err("URL must start with postgres:// or postgresql://".to_string());
        }
        
        let without_scheme = if url.starts_with("postgres://") {
            url.trim_start_matches("postgres://")
        } else {
            url.trim_start_matches("postgresql://")
        };
        
        // Split credentials+host from dbname
        let (credentials_host, dbname_and_params) = match without_scheme.split_once('/') {
            Some((left, right)) => (left, right),
            None => return Err("No database name in URL".to_string()),
        };
        
        // Extract database name and parameters
        let (dbname, params) = match dbname_and_params.split_once('?') {
            Some((name, params)) => (name, Some(params)),
            None => (dbname_and_params, None),
        };
        
        if dbname.is_empty() {
            return Err("Empty database name".to_string());
        }
        
        config.dbname = Some(dbname.to_string());
        
        // Process query parameters if present
        if let Some(params) = params {
            for param in params.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    match key {
                        "sslmode" => {
                            match value {
                                "require" => config.ssl_mode = Some(SslMode::Require),
                                "prefer" => config.ssl_mode = Some(SslMode::Prefer),
                                "disable" => config.ssl_mode = Some(SslMode::Disable),
                                _ => log::warn!("Unsupported sslmode: {}", value),
                            }
                        },
                        _ => {
                            // Ignore other parameters for now
                            log::debug!("Ignoring parameter: {}={}", key, value);
                        }
                    }
                }
            }
        }
        
        // Split credentials from host:port
        if let Some((credentials, host_port)) = credentials_host.split_once('@') {
            // Split username:password
            if let Some((username, password)) = credentials.split_once(':') {
                config.user = Some(username.to_string());
                config.password = Some(password.to_string());
            } else {
                config.user = Some(credentials.to_string());
            }
            
            // Split host:port
            if let Some((host, port)) = host_port.split_once(':') {
                config.host = Some(host.to_string());
                if let Ok(port_num) = port.parse::<u16>() {
                    config.port = Some(port_num);
                }
            } else {
                config.host = Some(host_port.to_string());
            }
        } else {
            // No credentials, just host:port
            if let Some((host, port)) = credentials_host.split_once(':') {
                config.host = Some(host.to_string());
                if let Ok(port_num) = port.parse::<u16>() {
                    config.port = Some(port_num);
                }
            } else {
                config.host = Some(credentials_host.to_string());
            }
            
            // Default credentials
            config.user = Some("postgres".to_string());
            config.password = Some("postgres".to_string());
        }
        
        Ok(())
    }
    
    fn default_db_config() -> PgConfig {
        let mut config = PgConfig::new();
        config.host = Some("localhost".to_string());
        config.port = Some(5432);
        config.dbname = Some("postgres".to_string());
        config.user = Some("postgres".to_string());
        config.password = Some("postgres".to_string());
        config
    }
}