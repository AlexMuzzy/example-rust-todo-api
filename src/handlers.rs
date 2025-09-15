use crate::database::DbPool;
use crate::models::{CreateTodo, Todo, UpdateTodo};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

// Get all todos
pub async fn todos_get_handler(State(pool): State<DbPool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
    {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch todos: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

// Get a single todo by id
pub async fn todo_get_handler(
    axum::extract::Path(id): axum::extract::Path<i32>,
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Todo not found")).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

// Create a new todo
pub async fn todo_post_handler(
    State(pool): State<DbPool>,
    Json(create_todo): Json<CreateTodo>,
) -> impl IntoResponse {
    let completed = create_todo.completed.unwrap_or(false);

    match sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed) VALUES ($1, $2) RETURNING *",
    )
    .bind(&create_todo.title)
    .bind(completed)
    .fetch_one(&pool)
    .await
    {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

// Update a todo
pub async fn todo_put_handler(
    axum::extract::Path(id): axum::extract::Path<i32>,
    State(pool): State<DbPool>,
    Json(update_todo): Json<UpdateTodo>,
) -> impl IntoResponse {
    // First, get the current todo to merge with updates
    let current_todo = match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(todo)) => todo,
        Ok(None) => return (StatusCode::NOT_FOUND, Json("Todo not found")).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch current todo: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response();
        }
    };

    // Merge updates with current values
    let title = update_todo.title.unwrap_or(current_todo.title);
    let completed = update_todo.completed.unwrap_or(current_todo.completed);

    match sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = $1, completed = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 RETURNING *"
    )
    .bind(&title)
    .bind(completed)
    .bind(id)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Todo not found")).into_response(),
        Err(e) => {
            tracing::error!("Failed to update todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

// Delete a todo
pub async fn todo_delete_handler(
    axum::extract::Path(id): axum::extract::Path<i32>,
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                (StatusCode::NO_CONTENT, Json("Todo deleted")).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json("Todo not found")).into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}
