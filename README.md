# cheomseongd

A lightweight Rust-based daemon tool that continuously watches KNU notice boards and delivers instant notifications for newly posted announcements.

## Run with Docker Compose

Create a `.env` file with `DB_USER` and `DB_PASSWORD`, then start the database and daemon:

```sh
docker compose up -d --build
docker compose run --rm app set --crawler computer --interval 300 --count 10
docker compose logs -f app
```

The application applies pending Diesel migrations when it starts. Crawler settings are stored in PostgreSQL and picked up by the running daemon. To stop the daemon while keeping PostgreSQL running, use `docker compose stop app`.

## Run without Docker

Set `DATABASE_URL`, apply migrations with `diesel migration run`, then use the CLI:

```sh
cargo run -- set --crawler computer --interval 300 --count 10
cargo run -- start
```
