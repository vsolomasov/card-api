# CardAPI

CardAPI - it's a project for learning the Rust.

## Local running

```bash
# Deploying infra to the dockers
docker-compose up -d

# Connecting to PG
docker exec -it -u postgres card-api-pg psql

# Starting CardAPI service
cargo run
```

## Generate secret key

```bash
echo $(xxd -g 2 -l 32 -p /dev/random | tr -d '\n')
```
