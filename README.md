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

## Testing on mobile

The dev server binds to `127.0.0.1` by default, which your phone can't reach even on the same WiFi. Use [cloudflared](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/do-more-with-tunnels/trycloudflare/) to get a temporary public HTTPS URL — no account needed.

Start the dev server bound to all interfaces:

```bash
ADDRESS=0.0.0.0:3000 make watch
```

In a second terminal, start the tunnel:

```bash
nix run nixpkgs#cloudflared -- tunnel --url http://localhost:3000
```

Look for a line like:

```
+--------------------------------------------------------------------------------------------+
|  Your quick Tunnel has been created! Visit it at (it may take a couple of minutes to be ready)  |
|  https://xxxx-xxxx-xxxx.trycloudflare.com                                                  |
+--------------------------------------------------------------------------------------------+
```

Open that URL on your phone. The tunnel is temporary and disappears when you stop the process.

## Deployment (NixOS)

Copy `database.json` to the server before first run. Set all env vars from `.env.example` in your service configuration, then start the service. The migrate binary reads from `DATABASE_JSON_PATH` and seeds quotes on startup — idempotent via `INSERT OR IGNORE`.
