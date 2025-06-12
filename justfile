# justfile for rust

set dotenv-load := true

# pre-commit

@setup:
    pre-commit install

# cargo

@build:
    cargo build

@run:
    cargo run

@test:
    cargo test

@lint:
    cargo clippy

@fmt:
    cargo fmt

@delete:
    rm -f .env custom/.env