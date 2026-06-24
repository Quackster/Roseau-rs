# Roseau Rust Port Progress

Goal: port the whole Roseau project to Rust using only safe Rust.

## Current Status

- Started a native Rust crate at the project root with `Cargo.toml` and `src/lib.rs`.
- Ported foundational protocol pieces from the Java Netty stack:
  - `NettyRequest` header/body splitting and argument access.
  - Decimal length-prefixed frame decoding using ISO-8859-1 byte mapping.
  - `NettyResponse` packet composition, delimiter helpers, object serialisation hook, finalisation, and debug body rendering.
- Ported foundational utility/configuration pieces:
  - INI-style configuration parsing for the existing `roseau.properties` shape.
  - Typed config lookups and boolean parsing.
  - Java-compatible input filtering and IPv4 validation helpers.
  - `GameSettings` constants plus a safe atomic item ID counter.
- Added focused Rust tests for the ported protocol, config, settings-adjacent utility behavior.

## Verification

- Pending after this edit: run `cargo test`.
- Pending after this edit: run a source scan for the forbidden Rust keyword.

## Remaining Porting Work

- Server bootstrap and runtime lifecycle (`Roseau`, logging, scheduler).
- TCP server implementation and connection/session management.
- DAO layer and database persistence.
- Game domain managers and entities.
- Room model, mapping, pathfinding, schedulers, and interactors.
- Incoming message handlers.
- Outgoing message composers.
- Test stubs and compatibility tests against Java behavior.
- Remove or archive Java/Gradle build once equivalent Rust coverage exists.
