ARG APP_NAME=autumn-homepage

# === Compile Rust App ===
FROM rust:1.82-alpine3.20 AS rust_stage
ARG APP_NAME
ENV APP_NAME=${APP_NAME}
WORKDIR /app

## Build Dependencies
RUN apk add --no-cache musl-dev libressl-dev \
    && rustup default nightly \
    && rustup target add wasm32-unknown-unknown \
    && cargo install dioxus-cli@0.6.0-alpha.5

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY entity ./entity
COPY migration ./migration

RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release --features server \
    && dx build --release

## Build Rust application
COPY Dioxus.toml ./
COPY assets ./assets
COPY src ./src

RUN cargo build --release --features server \
    && dx build --release

# === Generate Tailwindcss ===
FROM node:23.2-alpine3.20 AS node_stage
ARG APP_NAME
ENV APP_NAME=${APP_NAME}
WORKDIR /app

## Build Dependencies
COPY package.json package-lock.json ./

RUN npm i \
    && npm install -g tailwindcss@3.4.15 \
    && npm install --save-dev lightningcss-cli@1.28.1

## Generate & minify CSS
COPY tailwind.config.ts input.css ./
COPY src ./src

RUN npx tailwindcss -i ./input.css -o ./assets/tailwind.css \
    && npx lightningcss -m ./assets/tailwind.css -o ./assets/tailwind.css

# === Run application ===
FROM alpine:3.20
ARG APP_NAME
ENV APP_NAME=${APP_NAME}
WORKDIR /app

RUN apk add --no-cache ca-certificates curl

COPY --from=rust_stage /app/target/release/${APP_NAME} /app
COPY --from=rust_stage /app/target/dx/${APP_NAME}/release/web/public /app/public
COPY --from=node_stage /app/assets/tailwind.css /app/public/assets/tailwind.css

ENV IP="0.0.0.0"
ENV PORT=8080

EXPOSE 8080

ENV DATABASE_URL="sqlite://data/db.sqlite?mode=rwc"
CMD ["sh", "-c", "./${APP_NAME}"]
