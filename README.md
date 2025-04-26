# Rust User API with PostgreSQL

A RESTful API for user management built with Rust, Actix-Web, and PostgreSQL.

## Features

- RESTful CRUD operations for user management
- PostgreSQL database integration
- Containerized with Docker
- Environment-based configuration

## Project Structure

```
src/
├── main.rs             # Entry point
├── config.rs           # App configuration
├── models/
│   └── user.rs         # User model and DTOs
├── routes/
│   ├── mod.rs          # Routes module registration
│   └── user.rs         # User-related route handlers
└── repositories/
    ├── mod.rs          # Repository module registration
    └── user_repo.rs    # PostgreSQL-based user data access
```

## Prerequisites

- Rust (latest stable)
- PostgreSQL (or Docker & Docker Compose)

## Setup and Running

### Environment Variables

Copy the `.env.example` file to `.env` and adjust the values as needed:

```bash
cp .env.example .env
```

### Running with Docker

The easiest way to start the application is using Docker Compose:

```bash
docker-compose up -d
```

This will start both PostgreSQL and the application.

### Running Locally

1. Make sure PostgreSQL is running and accessible with the credentials specified in your `.env` file.

2. Build and run the application:

```bash
cargo run
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/users` | List all users |
| GET | `/users/{id}` | Get user by ID |
| POST | `/users` | Create new user |
| PUT | `/users/{id}` | Update user |
| DELETE | `/users/{id}` | Delete user |

## API Examples

### Create a User

```bash
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Smith", "email": "alice@example.com", "age": 28}'
```

### Get All Users

```bash
curl http://localhost:8080/users
```

### Get User by ID

```bash
curl http://localhost:8080/users/{user_id}
```

### Update a User

```bash
curl -X PUT http://localhost:8080/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Johnson", "age": 29}'
```

### Delete a User

```bash
curl -X DELETE http://localhost:8080/users/{user_id}
```

## Development

### Running Tests

```bash
cargo test
```

### Database Migrations

Database schema is automatically created when the application starts. The initial migration is in the `migrations` directory.