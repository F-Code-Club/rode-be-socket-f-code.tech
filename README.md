# Rode socket backend

## Run

- Set up a postgresql database

- Export database url, example (user, password, host, database can be changed to fit your postgresql instance)

````bash
export DATABASE_URL="postgres://user:password@host/database"
````

- Init the database, example (user can be changed to fit your postgresql instance)

``` bash
psql -d user -a -f ./schema.sql
```

- Run code

```bash
cargo run
```

## Test

```bash
cargo test
```
