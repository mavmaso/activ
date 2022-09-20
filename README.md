# Activ

depredado
`cargo install --version=0.5.5 sqlx-cli --no-default-features --features postgres`

`export DATABASE_URL=postgres://postgres:postgres@localhost:5432/activ_dev`

`sqlx database create`

`sqlx migrate add`
`sqlx migrate run`

`docker-compose up -d`

`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

`cargo test -- --nocapture`