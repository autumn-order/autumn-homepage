# Join Autumn

## Development

### Install Dependencies

- Install Rust <https://rustup.rs/>
- Install dioxus-cli `cargo install dioxus-cli`
- Install sea-orm-cli `cargo install sea-orm-cli`
- Install bun: <https://bun.sh/>
- Install Daisyui with `bun i`

### Run the Application

1. Run migrations with `sea-orm-cli migrate`
2. Run the application using these commands in 2 separate terminals

Watch for tailwindcss changes:

```bash
bunx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus app:

```bash
dx serve --platform fullstack
```

### Modifying Database Schema

After modifying the database schema run these commands

1. Run migrations: `sea-orm-cli migrate`
2. Generate entities: `sea-orm-cli generate entity -o ./entity/src/entities/ --date-time-crate chrono`
