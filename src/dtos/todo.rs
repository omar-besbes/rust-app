use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTodoDTO {
    pub title: String,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
