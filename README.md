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

## What's built



## TODO

- [x] SQLite database with repository pattern
- [x] Quotes seeded from JSON
- [x] Invite-only registration with expiring invite links
- [x] Passwordless login via OTP code
- [x] OTP resend with rate-aware random code generation
- [x] Session-based auth with tower-sessions
- [x] Auth middleware with custom Axum extractors (AuthUser, AdminUser)
- [x] Settings page (email toggle, send time)
- [x] Admin section — generate invite links via HTMX
- [x] Bootstrap admin user via Makefile
- [x] Askama templates with shared base layout
- [x] HTMX partial page updates on login, register, and settings flows
- [x] Resend API key integration (wired, not sending yet)
- [x] Tracing middleware
- [x] Delete account option in settings
- [x] Delete account confirmation (are you sure?)
- [x] Save settings confirmation (HTMX inline response)
- [x] Copy invite link to clipboard on click
- [x] Nice frontend
- [x] Schedule 15 min increments only
- [x] Handle timezone conversion for scheduling
- [x] Rate limiting
- [x] More logs
- [ ] Define `AppError` type implementing `IntoResponse` for 500 responses
- [ ] Return `Result` from all database functions instead of panicking
- [ ] Handle db errors in handlers with `?`
- [ ] Handle remaining unwraps in middleware and auth extractor
- [ ] Send login codes via email (currently logging only)
- [ ] Send quote emails via Resend
- [ ] Email scheduling — cron job to send quotes at user's `send_time`
- [ ] NixOS deployment — flake, systemd service, Cloudflare tunnel
