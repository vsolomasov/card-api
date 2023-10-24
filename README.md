# CardAPI

CardAPI - it's a project for learning the Rust.

## Crates

* `domain` - business logic
* `adapter` - connects the `domain` with input (http/grpc/etc) and output (database/s3/etc)
* `entrypoint` - bin crate
* `it` - integration tests
* `migrate` - database migration service
* `docs` - service documentation

## Generate secret key

```bash
echo $(xxd -g 2 -l 32 -p /dev/random | tr -d '\n')
```

## Local running

```bash
# Deploying infra to the docker
docker-compose up -d

# Connecting to PG
docker exec -it -u postgres card-api-pg psql

# Starting service
cargo run -p entrypoint

# Starting integration tests
cargo test -p it -- --ignored
```
