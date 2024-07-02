use sqlx::PgPool;

use crate::{dtos::todo::NewTodoDTO, schemas::todo::Todo};

pub async fn add_todo_service(pool: &PgPool, todo: NewTodoDTO) -> Result<Todo, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO todos (title, description, completed)
        VALUES ($1, $2, $3)
        RETURNING id, title, description, completed
        "#,
        todo.title,
        todo.description,
        todo.completed
    )
    .fetch_one(pool)
    .await?;

    Ok(Todo {
        id: row.id,
        title: row.title,
        description: row.description,
        completed: row.completed,
    })
}

pub async fn list_todos_service(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, description, completed
        FROM todos
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}
