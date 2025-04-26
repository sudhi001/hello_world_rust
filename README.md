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


# API Performance Benchmark Report

## Executive Summary
This report analyzes the performance of a REST API based on benchmark tests performed against four key endpoints. The benchmark executed 100 iterations per endpoint to gather statistically significant data on response times.

## Test Configuration
- **Host**: http://localhost:8080
- **Iterations**: 100 per endpoint
- **Testing Tool**: curl with time measurement
- **Endpoints Tested**: 
  - Health check (`/health`)
  - Get all users (`/users`)
  - Get user by ID (`/users/{id}`)
  - Create user (`POST /users`)

## Results

| Endpoint | Average Response Time |
|----------|----------------------:|
| Health check | 0.000771s |
| Get all users | 0.556659s |
| Get user by ID | 0.559870s |
| Create user | 0.566571s |

## Analysis

### Health Endpoint Performance
The health endpoint shows excellent performance at approximately 0.8 milliseconds per request. This is expected for a health check endpoint which typically:
- Performs minimal processing
- May not interact with the database
- Serves as a lightweight indicator of service availability

### User Endpoint Performance
All three user-related endpoints show remarkably consistent performance around 560 milliseconds per request:

1. **Get All Users (0.557s)**: Fastest of the user endpoints
2. **Get User by ID (0.560s)**: Slightly slower than get all users
3. **Create User (0.567s)**: Slowest endpoint, as expected for write operations

The similarity in response times across these endpoints suggests:
- They likely share common middleware or authentication mechanisms
- Database access patterns may be similar
- No significant optimization has been applied to any specific endpoint

### Performance Concerns
The response times for the user endpoints (approximately 560ms) are relatively high for a REST API, particularly for read operations. Several factors could contribute to this:

- Database connection overhead
- Inefficient queries
- Synchronous processing of requests
- Resource constraints in the testing environment
- Lack of caching mechanisms
- Heavyweight authentication or authorization checks

## Recommendations

1. **Implement Caching**: Add caching for read operations, particularly for the "get all users" endpoint
   
2. **Database Optimizations**:
   - Ensure proper indexing on user ID and frequently queried fields
   - Consider connection pooling if not already implemented
   - Review query performance and execution plans

3. **Code Profiling**:
   - Use profiling tools to identify bottlenecks in the request handling pipeline
   - Look for opportunities to optimize the most time-consuming operations

4. **Consider Pagination**:
   - Implement pagination for the "get all users" endpoint to improve response time as the dataset grows

5. **Asynchronous Processing**:
   - For write operations like user creation, consider using asynchronous processing where appropriate

6. **Load Testing**:
   - Perform load testing with concurrent users to understand how the API performs under scale
   - Monitor resource utilization during tests to identify potential hardware constraints

## Next Steps

1. Run more detailed profiling to identify specific bottlenecks
2. Implement the highest priority optimizations based on profiling results
3. Re-run benchmarks after each optimization to measure improvement
4. Expand testing to include concurrent requests to evaluate real-world performance

## Conclusion

While the API shows consistent performance across endpoints, there is significant room for optimization. The health endpoint performs as expected, but user endpoints would benefit from optimization efforts to bring response times down, particularly if this API is intended for production use with significant traffic.
