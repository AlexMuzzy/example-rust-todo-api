use crate::models::Todo;
use axum::{http::StatusCode, response::IntoResponse, Json};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// In-memory todo store
type TodoStore = Arc<RwLock<HashMap<i32, Todo>>>;

// Initialize the todo store
pub fn init_todo_store() -> TodoStore {
    let mut store = HashMap::new();
    store.insert(1, Todo {
        id: 1,
        title: "Buy milk".to_string(),
        completed: false,
    });
    store.insert(2, Todo {
        id: 2,
        title: "walk the dog".to_string(),
        completed: false,
    });
    Arc::new(RwLock::new(store))
}

// Get all todos
pub async fn todos_get_handler(store: &TodoStore) -> impl IntoResponse {
    let todos: Vec<Todo> = store.read().unwrap().values().cloned().collect();
    (StatusCode::OK, Json(todos))
}

// Get a single todo by id
pub async fn todo_get_handler(id: i32, store: &TodoStore) -> (axum::http::StatusCode, Json<&str>) {
    if let Some(todo) = store.read().unwrap().get(&id) {
        (StatusCode::OK, Json(todo.clone()))
    } else {
        (StatusCode::NOT_FOUND, Json("Todo not found"))
    }
}

// Create a new todo
pub async fn todo_post_handler(
    Json(todo): Json<Todo>,
    store: &mut TodoStore,
) -> impl IntoResponse {
    let id = todo.id;
    if store.contains_key(&id) {
        (StatusCode::CONFLICT, Json("Todo already exists"))
    } else {
        store.insert(id, todo.clone());
        (StatusCode::CREATED, Json(todo))
    }
}

// Update a todo
pub async fn todo_put_handler(
    id: i32,
    Json(updated_todo): Json<Todo>,
    store: &mut TodoStore,
) -> impl IntoResponse {
    if let Some(todo) = store.get_mut(&id) {
        *todo = updated_todo;
        (StatusCode::OK, Json(todo.clone()))
    } else {
        (StatusCode::NOT_FOUND, Json("Todo not found"))
    }
}

// Delete a todo
pub async fn todo_delete_handler(id: i32, store: &mut TodoStore) -> impl IntoResponse {
    if store.remove(&id).is_some() {
        (StatusCode::NO_CONTENT, "")
    } else {
        (StatusCode::NOT_FOUND, Json("Todo not found"))
    }
}