# Roseau-rs

Rust port of Roseau, a Habbo Hotel v1 server for the 2001 client revision.

The project contains the Rust server runtime, protocol handling, game domain code, MySQL persistence layer, and the Roseau seed database.

## Status

The Rust server can:

- decode and encode the v1 wire protocol;
- compose outgoing packets for the original client;
- dispatch incoming commands through typed Rust handlers;
- connect to MariaDB/MySQL through the `mysql` crate;
- load the seeded Roseau schema from `tools/roseau.sql`;
- bind the main hotel port, private room port, and every visible public room port;
- accept connections on every bound listener each server tick;
- run indefinitely from `cargo run`.

The public room ports follow the original Java Roseau behavior: the server listens on `server.port + room_id` for each enabled, non-hidden public room returned by the database.

## Requirements

- Rust stable toolchain
- MariaDB or MySQL reachable over TCP
- A Shockwave-capable Habbo v1 client if you want to connect an actual client

## Repository Layout

- `src/` - Rust server, game domain, protocol, runtime, DAO, and TCP code
- `tools/roseau.sql` - schema and seed data for the Roseau database
- `tools/dcr0910.zip` - client asset archive retained for local testing
- `PORT_PROGRESS.md` - porting status and validation notes
- `Cargo.toml` / `Cargo.lock` - Rust package metadata and locked dependencies

## Database Setup

Create and import the seed database:

```sh
mysql -h127.0.0.1 -P3306 -uroot -p -e \
  'CREATE DATABASE IF NOT EXISTS roseau CHARACTER SET latin1 COLLATE latin1_swedish_ci;'
mysql -h127.0.0.1 -P3306 -uroot -p roseau < tools/roseau.sql
```

Create a local `roseau.properties` if one does not already exist:

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

The binary creates default `roseau.properties` and `habbohotel.properties` if they are missing. Local config files and runtime logs are ignored by git.

## Build And Test

```sh
cargo fmt --check
cargo check --bin roseau-rs
cargo test
```

## Run

Start the server:

```sh
cargo run
```

On startup, the server validates database connectivity, queries public room IDs, binds all configured listener ports, prints startup logs, then continues running until the process is stopped.

Example startup listener lines:

```text
Server is listening on 127.0.0.1:37120
Public room 5 is listening on 127.0.0.1:37125
Public room 7 is listening on 127.0.0.1:37127
```

The only runtime options select config file locations:

```sh
cargo run -- --main-config roseau.properties --hotel-config habbohotel.properties
```

A basic TCP smoke test against a bound listener should receive:

```text
#HELLO##
```

## Notes

- Main hotel listener: `server.port`.
- Private room listener: `server.private.port`.
- Public room listeners: `server.port + room_id` for every enabled, visible public room.
- Connections are always accepted; there is no max-tick, listener-index, or accept/skip mode in the CLI.
- Broader live-flow testing is still needed for login, room entry, inventory, catalogue, messenger, moderation, and disconnect behavior.
