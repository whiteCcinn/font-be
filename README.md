# font-be

# pgsql

```shell
docker run -d \
    --name pgsql \
    -e POSTGRES_PASSWORD=123456 \
    -e PGDATA=/var/lib/postgresql/data/pgdata \
    -v data:/var/lib/postgresql/data \
    -p 13307:5432 \
    postgres:latest
```