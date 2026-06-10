include .env

setup:
	sqlx database create
	sqlx migrate run
	cargo run --bin migrate
	$(MAKE) bootstrap

reset:
	sqlx database drop -y
	$(MAKE) setup

bootstrap:
	sqlite3 $(shell echo $(DATABASE_URL) | sed 's|sqlite://||') "INSERT OR IGNORE INTO users (email, is_admin) VALUES ('$(BOOTSTRAP_ADMIN_EMAIL)', 1);"

watch:
	cargo watch -x run
