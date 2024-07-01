# Todo app

A simple todo app using rust.

## How to run ?

### Fastest way

Make sure you have [docker](https://docs.docker.com/get-docker/) installed on your machine.

1. Copy `.env.example` to `.env`. Feel free to make changes if you have to.
2. Run `docker compose up -d`.

You now have the app up and running at `http://localhost:8080`.

### Run the app locally and the rest in docker

Make sure you have [docker](https://docs.docker.com/get-docker/) and [cargo](https://rustup.rs) installed on your machine.

1. Copy `.env.example` to `.env`. Feel free to make changes if you have to.
2. Run this:

```sh
docker compose up -d db dbviewer
cargo install sqlx-cli --features postgres
cargo sqlx migrate run
bash -c 'source .env && DATABASE_URL=$DATABASE_URL cargo run'
```

## How to try ?

There are 2 endpoints in the app.

- `GET /todos`: to get all todos in the database.
- `POST /todos`: to add a todo. The body should at least contain `title` and `description`.

This is how to get all todos using curl:

```sh
curl http://localhost:8080/todos
```

This is how to add a todo using curl:

```sh
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
         "title": "My Todo",
         "description": "This is a sample todo description."
     }' \
     http://localhost:8080/todos
```

You can open `http://localhost:3000`. To directly query the db from `sqlpad` (Don't forget to choose the connection in the upper left corner or you won't see anything).
