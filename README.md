# Daily Stoic

Invite-only web app for receiving daily stoic quotes via email. Built with Rust, Axum, SQLite, Askama, and HTMX. Passwordless login via email codes.

## Setup

Copy `.env.example` to `.env` and fill in the values. `DATABASE_JSON_PATH` points to the quotes source file — for local development use `./database.json`. Then:

```bash
make setup
```

This creates the database, runs migrations, seeds quotes, and bootstraps the admin user from `BOOTSTRAP_ADMIN_EMAIL`.

To reset:

```bash
make reset
```

## Development

```bash
make watch
```

## Deployment (NixOS)

Copy `database.json` to the server before first run. Set all env vars from `.env.example` in your service configuration, then start the service. The migrate binary reads from `DATABASE_JSON_PATH` and seeds quotes on startup — idempotent via `INSERT OR IGNORE`.
