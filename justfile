set dotenv-load := true

## Development
# Start the server
dev-start:
    cargo run

# Start the server with hot-module reload / swap
dev-watch:
    cargo watch -x run

## Quality assurance
# Checks for inconsistencies, warnings and errors in the codebase
inspect:
    @echo "Checking the repository for:"
    @echo "Formatting issues (rustfmt)"
    cargo fmt --all -- --check
    @echo "Linting violations (clippy)"
    cargo clippy -- -D warnings
    @echo "Failing tests (cargo test)"
    cargo test
    @echo "Dependency vulnerabilities in Cargo.lock file (cargo-audit)"
    cargo audit

## Docker services
# Connect to PostgreSQL via `psql`
db-connect:
    docker exec -it cadence-postgres psql -U $POSTGRES_USER -d cadence

# Spawn an interactive bash session inside of the database service
db-shell:
    docker exec -it cadence-postgres bash

## SQLx database commands
# Create database specified in DATABASE_URL
db-create:
    sqlx database create

# Drop database specified in DATABASE_URL
db-drop:
    sqlx database drop -y

# Drops, creates and seeds the database specified in DATABASE_URL
db-reset:
    sqlx database reset -y && psql $DATABASE_URL -f ./scripts/seed-db.sql
