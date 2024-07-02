use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct NewTodoDTO {
    #[validate(length(
        min = 3,
        max = 200,
        message = "Title lenght must be between 3 and 200 characters"
    ))]
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub completed: bool,
}
