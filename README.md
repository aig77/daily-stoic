# Daily Stoic

Invite-only web app for receiving daily stoic quotes via email. Built with Rust, Axum, SQLite, Askama, and HTMX. Passwordless login via email codes.

## Setup

Copy `.env.example` to `.env` and fill in the values, then:

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

## Server setup

`database.json` is not included in the repo. Before starting the service for the first time, copy it to the server's state directory:

```bash
scp database.json root@your-server:/var/lib/daily-stoic/database.json
systemctl restart daily-stoic
```

The migrate binary reads from `DATABASE_JSON_PATH` (defaults to `/var/lib/daily-stoic/database.json`) and seeds quotes on every startup — safely idempotent via `INSERT OR IGNORE`.

## TODO

- [ ] Notification email to admins when a new user signs up
- [ ] Captcha after clicking login
- [ ] Add limit to number of times people can change the schedule to 3/day
- [ ] Define `AppError` type implementing `IntoResponse` for 500 responses
- [ ] Return `Result` from all database functions instead of panicking
- [ ] Handle db errors in handlers with `?`
- [ ] Handle remaining unwraps in middleware and auth extractor
