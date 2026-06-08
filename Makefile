setup:
	sqlx database create
	sqlx migrate run
	cargo run --bin migrate

reset:
	sqlx database drop -y
	sqlx database create
	sqlx migrate run
	cargo run --bin migrate

watch:
	cargo watch -x run
