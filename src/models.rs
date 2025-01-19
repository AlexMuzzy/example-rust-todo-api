use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) completed: bool,
}