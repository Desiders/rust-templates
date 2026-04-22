help:
    just -l

fmt:
    just -f web-template/justfile fmt
    just -f tg-bot-template/justfile fmt

lint:
    just -f web-template/justfile lint
    just -f tg-bot-template/justfile lint

check:
    cargo check --workspace
