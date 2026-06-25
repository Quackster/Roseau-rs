# Roseau-rs

Rust port of Roseau, a Habbo Hotel v1 server for the 2001 client revision.

The repository contains the Rust server implementation, runtime, protocol handling, MySQL persistence layer, and seed database dump. It is managed as a Rust project with Cargo.

## Status

The Rust server can:

- decode and encode the v1 wire protocol;
- compose outgoing packets for the original client;
- dispatch incoming commands through typed Rust handlers;
- run bounded TCP listener ticks using the standard library;
- connect to MariaDB/MySQL through the `mysql` crate;
- load the seeded Roseau schema from `tools/roseau.sql`;
- start locally, bind `127.0.0.1:37120`, accept a TCP client, and send `#HELLO##`.

The port is still being hardened. See [PORT_PROGRESS.md](PORT_PROGRESS.md) for the detailed migration log, verification notes, and remaining work.

## Requirements

- Rust stable toolchain
- MariaDB or MySQL reachable over TCP
- A Shockwave-capable Habbo v1 client if you want to connect an actual client

## Repository Layout

- `src/` - Rust server, game domain, protocol, runtime, DAO, and TCP code
- `tools/roseau.sql` - schema and seed data for the Roseau database
- `tools/dcr0910.zip` - client asset archive retained for local testing
- `PORT_PROGRESS.md` - porting status and remaining validation work
- `Cargo.toml` / `Cargo.lock` - Rust package metadata and locked dependencies

## Database Setup

Create and import the seed database:

```sh
mysql -h127.0.0.1 -P3306 -uroot -p -e \
  'CREATE DATABASE IF NOT EXISTS roseau CHARACTER SET latin1 COLLATE latin1_swedish_ci;'
mysql -h127.0.0.1 -P3306 -uroot -p roseau < tools/roseau.sql
```

Create a local `roseau.properties` if one does not already exist. The binary can generate defaults, but for the Rust server handler the important values are:

```ini
[Server]
server.ip=127.0.0.1
server.port=37120
server.private.port=37119
server.class.path=roseau::server::ServerHandler

[Database]
type=mysql
hostname=127.0.0.1
port=3306
username=root
password=verysecret
database=roseau
options=

[Logging]
log.errors=true
log.output=true
log.connections=true
log.packets=true
```

`roseau.properties`, `habbohotel.properties`, and runtime logs are local files and are ignored by git.

## Build And Test

```sh
cargo fmt --check
cargo check --bin roseau-rs
cargo test
```

## Run

Run one bounded server tick without accepting a client:

```sh
cargo run -- --max-ticks 1 --no-accept-connection
```

Run a longer bounded loop that can accept a local client:

```sh
cargo run -- --max-ticks 1000000 --accept-connection
```

A basic TCP smoke test should receive:

```text
#HELLO##
```

## Notes

- The binary validates database connectivity before listener startup.
- The runtime loop is bounded through CLI options while the port is being validated.
- Broader live-flow testing is still needed for login, room entry, inventory, catalogue, messenger, moderation, and disconnect behavior.
