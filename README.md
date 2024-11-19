# Autumn Homepage
The homepage for EVE Online corporations [The Order of Autumn](https://zkillboard.com/corporation/98785281/) & [Autumn Highsec Division](https://zkillboard.com/corporation/98784256/), part of [Black Rose](https://black-rose.space/) alliance & Phoenix Coalition.

This is a fullstack [Rust](https://www.rust-lang.org/) application built using [Dioxus v0.6.0-alpha.5](https://dioxuslabs.com/).

## Development

Enable rust-analyzer feature `"server"` for your code editor to include backend code for your Rust lanaguage server:

```json
"rust-analyzer.cargo.features": ["server"]
```



### Install Dependencies

- Install Rust: <https://rustup.rs/>
- Install bun: <https://bun.sh/>

Install dioxus-cli

```bash
cargo install dioxus-cli@0.6.0-alpha.5
```
Install sea-orm-cli

```bash
cargo install sea-orm-cli
```

Install nodejs dependencies (DaisyUi) with

```bash
bun i
```

### Run the Application

1. Copy `.env.example` to `.env` and set the `APPLICATION_EMAIL` variable to your contact email
2. Run migrations with

```bash
sea-orm-cli migrate
```

3. Run the application using these commands in 2 separate terminals

```bash
bunx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

```bash
dx serve
```

### Running Tests

```bash
cargo test --features server
```

### Modifying Database Schema

After modifying the database schema run these commands in order

```bash
sea-orm-cli migrate
```

```bash
sea-orm-cli generate entity -o ./entity/src/entities/ --date-time-crate chrono
```
