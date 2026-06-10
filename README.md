# Daily Stoic

Invite-only web app for receiving daily stoic quotes via email. Built with Rust, Axum, SQLite, Askama, and HTMX.

## Setup

Copy `.env.example` to `.env` and fill in the values, then:

```bash
make setup
```

This creates the database, runs migrations, seeds quotes, and bootstraps the admin user from `BOOTSTRAP_ADMIN_EMAIL`.

## Development

```bash
make watch
```

## TODO
-[ ] Auth middleware using custom extractor
-[ ] Delete account settings option
-[ ] Rate limiting
-[ ] Admin page
-[ ] Save settings confirmation (HTMX inline response)
-[ ] Send verification emails via resend
-[ ] Email scheduling - cron job to send quotes at user's `send_time`
-[ ] Copy invite link to clipboard on click

