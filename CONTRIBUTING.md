# Contributing

Contributions are always welcome to Autumn repositories, feel free to open an issue or create a pull request if you see something that could be improved.
Remember to always be respectful & constructive when assisting others with their contributions.

## Running in Development

Enable rust-analyzer feature `"server"` for your code editor to include backend code for your Rust lanaguage server:

```json
"rust-analyzer.cargo.features": ["server"]
```

### Install Dependencies

- Install Rust: <https://rustup.rs/>
- Install nodejs: <https://nodejs.org/en>

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
npm i
```

### Run the Application

1. Copy `.env.example` to `.env` and set the `APPLICATION_EMAIL` variable to your contact email
2. Run the application using these commands in 2 separate terminals

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

```bash
dx serve
```

Migrations are ran automatically during startup, ensure the ./data directory exists or the application will not be able to create the sqlite database file.

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
