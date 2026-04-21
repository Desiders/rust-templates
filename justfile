help:
    just -l

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy --all-features -- -W clippy::pedantic

run:
    cargo build --bin web_template && cargo run --bin web_template

@run-dev:
    CONFIG_PATH='configs/dev.toml' just run

@run-prod:
    CONFIG_PATH='configs/prod.toml' just run

@docker-dev-up:
    docker compose -f deployment/docker-compose.yaml --env-file .env --profile dev up -d
