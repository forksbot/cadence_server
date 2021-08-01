## Development
# Start the server
dev-start:
    cargo run

# Start the server with hot-module reload / swap
dev-watch:
    cargo watch -x run

## Docker services
# Connect to PostgreSQL via `psql`
db-connect:
    docker exec -it timesync-postgres psql -U $POSTGRES_USER -d postgres

# Spawn an interactive bash session inside of the database service
db-shell:
    docker exec -it timesync-postgres bash
