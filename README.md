# font-be


# install

```shell

# install db service
docker run -d \
    --name pgsql \
    -e POSTGRES_PASSWORD=123456 \
    -e PGDATA=/var/lib/postgresql/data/pgdata \
    -v $(PWD)/pgsql/data/:/var/lib/postgresql/data \
    -v $(PWD)/pgsql/init/init-database-db.sh:/docker-entrypoint-initdb.d/init-database-db.sh \
    -p 13307:5432 \
    postgres:latest

# install orm-dev component
cargo install diesel_cli --no-default-features --features postgres

# install table for db
diesel migration run

# insert data
cargo run --bin insert_fonts

# start

cargo run
```

Open browser access `http://localhost:5678/`

# API

- http://localhost:5678/search/{word}
- http://localhost:5678/test
- http://localhost:5678/