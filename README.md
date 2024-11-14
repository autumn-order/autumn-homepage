# Autumn Homepage
The homepage for EVE Online corporations [The Order of Autumn](https://zkillboard.com/corporation/98785281/) & [Autumn Highsec Division](https://zkillboard.com/corporation/98784256/), part of [Black Rose](https://black-rose.space/) alliance & Phoenix Coalition.

This is a fullstack [Rust](https://www.rust-lang.org/) application built using [Dioxus v0.6.0-alpha.4](https://dioxuslabs.com/).

## Development

Enable rust-analyzer feature `"server"` for your code editor to include backend code for your Rust lanaguage server.

`"rust-analyzer.cargo.features": ["server"]`

### Install Dependencies

- Install Rust <https://rustup.rs/>
- Install dioxus-cli `cargo install dioxus-cli@0.6.0-alpha.4`
- Install sea-orm-cli `cargo install sea-orm-cli`
- Install bun: <https://bun.sh/>
- Install nodejs dependencies (DaisyUi) with `bun i`

### Run the Application

1. Run migrations with `sea-orm-cli migrate`
2. Run the application using these commands in 2 separate terminals

Watch for tailwindcss changes:

```bash
bunx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus app:

```bash
dx serve
```

### Modifying Database Schema

After modifying the database schema run these commands

1. Run migrations: `sea-orm-cli migrate`
2. Generate entities: `sea-orm-cli generate entity -o ./entity/src/entities/ --date-time-crate chrono`
