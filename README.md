![Autumn Banner](https://raw.githubusercontent.com/autumn-order/branding/refs/heads/main/autumn-github-banner.png)

# Autumn Homepage
The homepage for EVE Online corporations [The Order of Autumn](https://zkillboard.com/corporation/98785281/) & [Autumn Highsec Division](https://zkillboard.com/corporation/98784256/), part of [Black Rose](https://black-rose.space/) alliance & Phoenix Coalition.

This is a fullstack [Rust](https://www.rust-lang.org/) application built using [Dioxus v0.6.0-alpha.5](https://dioxuslabs.com/).

## Running in Production

Ensure your server has plenty of resources to build the application, since it is written in Rust the application will need to compile first.
If you are constrained on server resources consider building the application on your local computer first and then pushing the built application/docker image to the server.

### Docker

Docker is the recommended and easiest approach for running this application in production.

- Install docker: <https://www.docker.com/>

Running the Application
1. Copy `.env.example` to `.env` and set the `APPLICATION_EMAIL` variable to your contact email for [ESI](https://esi.evetech.net/) requests.
2. Run the application using

```bash
docker-compose up -d
```

For locally testing the docker-compose setup at `localhost:8080` ensure you uncomment the ports in the `docker-compose.yml` file.

### Manual

- Install Rust: <https://rustup.rs/>
- Install nodejs: <https://nodejs.org/en>

Install dioxus-cli

```bash
cargo install dioxus-cli@0.6.0-alpha.5
```

Install nodejs dependencies (DaisyUi) with

```bash
npm i
```

1. Copy `.env.example` to `.env` and set the `APPLICATION_EMAIL` variable to your contact email for [ESI](https://esi.evetech.net/) requests.
2. Generate tailwindcss

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css
```

3. Build the web assets and then the backend server

```bash
dx build --release
```

```bash
cargo build --features server --release
```

4. Move the compiled application to a location for running it such as `/var/www/autumn-homepage`

```bash
mkdir -p /var/www/autumn-homepage/data
cp .env /var/www/autumn-homepage/.env
cp ./target/dx/autumn-homepage/release/web/public /var/www/autumn-homepage/public
cp ./target/release/autumn-homepage /var/www/autumn-homepage/autumn-homepage
```

5. Run the application

```bash
/var/www/autumn-homepage/autumn-homepage
```

You will need to configure systemd to automatically start the application whenever the server restarts.

Additionally you may need to setup a reverse proxy such as [nginx](https://nginx.org/en/), [traefik](https://traefik.io/), or [river](https://github.com/memorysafety/river) if you are running multiple websites on the same server.

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
