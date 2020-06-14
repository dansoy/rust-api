## REST API with Rust using Actix-Web 2.0

##### API Endpoints:

- GET /users ➡ returns all users
- POST /users ➡ takes in a JSON payload and creates a new user based on it
- PUT /users ➡ takes in a JSON payload and updates the user
- GET /users/{id} ➡ returns the user with a given id
- DELETE /users/{id} ➡ deletes the user with a given id
- POST /register ➡ takes in a JSON payload and creates a new user based on it
- POST /auth ➡ takes in a JSON payload for login
- DELETE /auth ➡ for logout
- Get /auth ➡ returns the user data if we are signed in and a status 401 if we are not

##### Crates Used

- [actix-redis](https://crates.io/crates/actix-redis)
- [actix-rt](https://crates.io/crates/actix-rt)
- [actix-session](https://crates.io/crates/actix-session)
- [actix-web](https://crates.io/crates/actix-web)
- [chrono](https://crates.io/crates/chrono)
- [derive_more](https://crates.io/crates/derive_more)
- [diesel](https://crates.io/crates/diesel)
- [diesel_migrations](https://crates.io/crates/diesel_migrations)
- [dotenv](https://crates.io/crates/dotenv)
- [env_logger](https://crates.io/crates/env_logger)
- [lazy_static](https://docs.rs/lazy_static)
- [log](https://docs.rs/log/0.4.8/log/)
- [rand](https://docs.rs/rand/0.7.3/rand/)
- [rust-argon2](https://docs.rs/rust-argon2/0.8.2/argon2/)
- [serde](https://crates.io/crates/serde)
- [serde_derive](https://crates.io/crates/serde_derive)
- [serde_json](https://crates.io/crates/serde_json)
- [uuid](https://crates.io/crates/uuid)

## Dependencies

##### Install Diesel CLI:

- Click [here](https://steemit.com/programming/@mrblueberry/installing-rust-and-diesel-for-rocket-on-windows-10) for reference on how to install diesel_cli
```
cargo install diesel_cli
```
## How To Use

- clone repository

```
git clone https://github.com/dansoy/rust-api project-name
```
- cd into your project

```
cd project-name
```

- create a copy of your .env file

```
cp .env.example .env
```

- create an empty Mysql database and add your database details to the .env file

```
DB_HOST=
DB_PORT=
DB_DATABASE=
DB_USERNAME=
DB_PASSWORD=
```

- run server

```
cargo run
```