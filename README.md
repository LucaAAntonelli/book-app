To set up Docker:
```
docker pull postgres
docker run --name bookdb -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres
```

Then, set the SQL schema:

```
psql -h localhost -U postgres -d postgres -f src/schema.sql
```

To set up sqlx:

```
cargo sqlx prepare --database-url postgres://postgres:mysecretpassword@localhost/postgres
```