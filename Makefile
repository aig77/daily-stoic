setup:
	sqlx migrate run
	cargo run --bin migrate
