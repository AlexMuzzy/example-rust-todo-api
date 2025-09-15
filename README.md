# Rust Todo API with PostgreSQL

A simple REST API for managing todos built with Rust, Axum, and PostgreSQL.

## Features

- CRUD operations for todos
- PostgreSQL database with SQLx
- Docker Compose setup
- Automatic database migrations
- Structured logging with tracing

## API Endpoints

- `GET /` - Get all todos
- `GET /{id}` - Get a specific todo by ID
- `POST /` - Create a new todo
- `PUT /{id}` - Update a todo
- `DELETE /{id}` - Delete a todo

## Quick Start

### Using Docker Compose (Recommended)

1. Clone the repository
2. Run the application with Docker Compose:
   ```bash
   docker-compose up --build
   ```

This will start:
- PostgreSQL database on port 5432
- Rust API server on port 8080

### Local Development

1. Install dependencies:
   ```bash
   cargo build
   ```

2. Set up environment variables:
   ```bash
   export DATABASE_URL="postgres://todo_user:todo_password@localhost:5432/todo_db"
   ```

3. Start PostgreSQL (using Docker):
   ```bash
   docker run --name todo_postgres -e POSTGRES_DB=todo_db -e POSTGRES_USER=todo_user -e POSTGRES_PASSWORD=todo_password -p 5432:5432 -d postgres:15-alpine
   ```

4. Run the application:
   ```bash
   cargo run
   ```

## API Usage Examples

### Create a todo
```bash
curl -X POST http://localhost:8080/ \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "completed": false}'
```

### Get all todos
```bash
curl http://localhost:8080/
```

### Update a todo
```bash
curl -X PUT http://localhost:8080/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "completed": true}'
```

### Delete a todo
```bash
curl -X DELETE http://localhost:8080/1
```

## Database Schema

The `todos` table has the following structure:
- `id` (SERIAL PRIMARY KEY)
- `title` (VARCHAR(255) NOT NULL)
- `completed` (BOOLEAN NOT NULL DEFAULT FALSE)
- `created_at` (TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)
- `updated_at` (TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)

## Project Structure

- `src/main.rs` - Application entry point and route configuration
- `src/handlers.rs` - HTTP request handlers
- `src/models.rs` - Data models and DTOs
- `src/database.rs` - Database connection and migration setup
- `migrations/` - SQL migration files
- `docker-compose.yml` - Docker Compose configuration
- `Dockerfile` - Application container configuration
