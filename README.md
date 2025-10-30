# Daily Stoic API

A Rust-based REST API for accessing quotes from "The Daily Stoic" by Ryan Holiday. Built with Axum and designed for performance, safety, and simplicity.

## Features

- 📚 Access stoic wisdom quotes by date
- 🎲 Get random quotes for daily inspiration
- 📅 Retrieve today's quote automatically
- 🔄 Update quotes via REST API
- 🦀 Built with Rust for memory safety and performance
- ⚡ Async runtime powered by Tokio

## Prerequisites

- Rust 2024 edition or later
- Cargo (comes with Rust)

Or use Nix flakes for reproducible development environment:

```bash
# Enter development shell
nix develop

# Or with direnv
direnv allow
```

## Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd daily-stoic-api-rs

# Build the project
cargo build

# Run tests
cargo test

# Run the server
cargo run
```

## Development

### With cargo-watch (auto-reload on file changes)

```bash
# Run with auto-reload
cargo watch -x run
```

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality:

- **rustfmt** - Automatically formats code
- **clippy** - Runs linter checks

Hooks run automatically on `git commit`. To run manually:

```bash
cargo fmt
cargo clippy
```

### Available Commands

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Lint
cargo clippy

# Format
cargo fmt

# Type check
cargo check
```

## API Endpoints

### Root
- **GET** `/` - Status page with API documentation

### Quotes
- **GET** `/quote/{id}` - Get a quote by date ID (MM-DD format)
  - Example: `/quote/03-15` for March 15th
  
- **GET** `/quote/daily` - Get today's stoic quote
  
- **GET** `/quote/random` - Get a random stoic quote
  
- **PUT** `/quote/{id}` - Update a quote by date ID
  - Requires JSON body with quote data

## Example Requests

### Get Quote by ID
```bash
curl http://localhost:3000/quote/03-15
```

### Get Daily Quote
```bash
curl http://localhost:3000/quote/daily
```

### Get Random Quote
```bash
curl http://localhost:3000/quote/random
```

### Update Quote
```bash
curl -X PUT http://localhost:3000/quote/03-15 \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2024-03-15",
    "title": "On Wisdom",
    "quote": "The obstacle is the way",
    "quoter": "Marcus Aurelius",
    "explanation": "Every obstacle presents an opportunity for growth"
  }'
```

## Response Format

All quote endpoints return JSON in the following format:

```json
{
  "date": "2024-03-15",
  "title": "On Wisdom",
  "quote": "The obstacle is the way",
  "quoter": "Marcus Aurelius",
  "explanation": "Every obstacle presents an opportunity for growth"
}
```

## Database

The API uses a JSON file (`database.json`) for storing quotes. The database is loaded into memory at startup for fast access and uses mutex-based synchronization for thread-safe concurrent access.

### Database Format

```json
{
  "03-15": {
    "date": "2024-03-15",
    "title": "On Wisdom",
    "quote": "The obstacle is the way",
    "quoter": "Marcus Aurelius",
    "explanation": "Every obstacle presents an opportunity for growth"
  }
}
```

## Project Structure

```
daily-stoic-api-rs/
├── src/
│   ├── models/          # Data models (Quote, DateId)
│   ├── routes/          # HTTP route handlers
│   ├── services/        # Business logic (database operations)
│   ├── lib.rs          # Library exports
│   └── main.rs         # Application entry point
├── tests/
│   └── api_tests.rs    # Integration tests
├── Cargo.toml          # Dependencies and project metadata
└── README.md           # This file
```

## Testing

```bash
# Run all tests
cargo test
```

## Code Style

- **Edition**: Rust 2024
- **Unsafe code**: Forbidden (enforced via workspace lints)
- **Formatting**: Auto-formatted via `cargo fmt`
- **Linting**: Enforced via `cargo clippy`
- **Error handling**: Uses `Result<T, E>` for fallible operations
- **Async**: Uses Tokio runtime for async handlers

## License

MIT

## Acknowledgments

- Quotes from "The Daily Stoic" by Ryan Holiday
- Built with [Axum](https://github.com/tokio-rs/axum)
- Powered by [Tokio](https://tokio.rs/)
