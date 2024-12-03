![Autumn Banner](https://raw.githubusercontent.com/autumn-order/branding/refs/heads/main/autumn-github-banner-dark.png#gh-light-mode-only)
![Autumn Banner](https://raw.githubusercontent.com/autumn-order/branding/refs/heads/main/autumn-github-banner-light.png#gh-dark-mode-only)

# Autumn Homepage
The homepage for EVE Online corporations [The Order of Autumn](https://zkillboard.com/corporation/98785281/) & [Autumn Highsec Division](https://zkillboard.com/corporation/98784256/), part of [Black Rose](https://black-rose.space/) alliance & Phoenix Coalition.

This is a fullstack [Rust](https://www.rust-lang.org/) application built using [Dioxus](https://dioxuslabs.com/).

For contributions & development instructions please see [CONTRIBUTING.md](./CONTRIBUTING.md).

## Running in Production

Ensure your server has plenty of resources to build the application, since it is written in Rust the application will need to compile first.
If you are constrained on server resources consider building the application on your local computer first and then pushing the built docker image to the server.

### Docker

Docker is the recommended and easiest approach for running this application in production.

- Install docker for your respective operating system: <https://docs.docker.com/engine/install/>

Running the Application
1. Copy `.env.example` to `.env` and set the `ESI_CONTACT_EMAIL` variable to your contact email for [ESI](https://esi.evetech.net/) requests.
2. Run the application using

```bash
docker-compose up -d
```
