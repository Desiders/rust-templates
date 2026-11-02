# Web template

A Rust REST API template with Axum, Froodi, PostgreSQL, and OpenAPI documentation.

## Run locally

Requires Rust, Docker, and `just`. Run these commands from `web-template/`.

Copy the configuration:

```sh
cp .env.template .env
cp configs/dev.template.toml configs/dev.toml
```

Set matching database credentials in both files. Start PostgreSQL, apply
migrations, and run the API:

```sh
just docker-dev-up
just docker-migrate up
just run-dev
```

The default address is `http://127.0.0.1:3001`.

## API

- `/api/v1/users` — example user endpoints.
- `/docs` — interactive API documentation.
- `/api-docs/openapi.json` — OpenAPI schema.
- `/ping` and `/status` — health checks.

## OpenAPI schema

Regenerate `openapi.yaml` after changing the API:

```sh
just openapi
```

Check that the schema is up to date:

```sh
just openapi-check
```
