setup:
	sqlx database create
	sqlx migrate run
	cargo run --bin migrate
