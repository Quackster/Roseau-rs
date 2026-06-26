# Roseau-rs

Roseau-rs is a Rust port of [Quackster/Roseau](https://github.com/Quackster/Roseau), targeting the original Habbo Hotel V1 client protocol from the early 2001 era. It implements the classic text-based client packet flow, login and registration handling, room navigation, messenger features, catalogue and inventory behaviour, room interactions, and a TCP runtime around the old hotel/private room socket model.

This project is intentionally close to the original server design, but written as a Rust crate with small protocol, runtime, DAO, message, and game-planning modules.


## Requirements

- Rust 2021 toolchain.
- MySQL-compatible database.
- A Habbo Hotel V1-compatible client and assets.

The included config files default to `127.0.0.1`, main server port `37120`, private room port `37119`, and database name `roseau`.

## Running

Build and test:

```sh
cargo build
cargo test
```

Create and populate the database using `tools/roseau.sql`, then update `roseau.properties` with the correct database host, username, password, and database name.

Start the server:

```sh
cargo run
```

Or pass explicit config paths:

```sh
cargo run -- --main-config roseau.properties --hotel-config habbohotel.properties
```

Available runtime flags:

```text
--main-config <path>
--hotel-config <path>
-h, --help
```

## Configuration

`roseau.properties` controls server binding, database access, and logging:

- `server.ip` is the bind address.
- `server.port` is the main hotel socket.
- `server.private.port` is the private room socket.
- `type=mysql` selects MySQL storage.
- `log.connections` and `log.packets` enable connection and packet logging.

`habbohotel.properties` controls game-facing defaults:

- registration character set and starting credits,
- periodic credit scheduler,
- bot response timing,
- drink carrying and talking timers,
- AFK room kick timing,
- debug mode.

## Protocol

Incoming TCP data is decoded by `NetworkFrameDecoder` into `NettyRequest` values. A request has a packet header such as `LOGIN`, `VERSIONCHECK`, `CHAT`, or `GETSTRIP`, plus the remaining body and arguments.

Outgoing packets are represented by `OutgoingMessage` implementations. They write into `NettyResponse`, which handles the classic response format used by the V1-era client.

## Expansion Guides

The guides below are collapsed so the README stays scannable. Open the topic that matches the feature you are adding.

<details>
<summary>Add an incoming packet handler</summary>

Most client packets are added under `src/messages/incoming/`.

1. Create a handler file, for example `src/messages/incoming/wave.rs`.
2. Implement `IncomingEvent` for a small handler type.
3. Parse the request through the `ClientMessage` methods.
4. Either send an immediate response with `context.send(...)` or record an `IncomingCommand` with `context.record(...)`.
5. Export the module from `src/messages/incoming/mod.rs`.
6. Register the packet header in `src/messages/message_handler.rs`.
7. Add a focused test next to the handler.

```rust
use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Wave;

impl IncomingEvent for Wave {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if !context.is_in_room() {
            return;
        }

        context.record(IncomingCommand::SendAlert {
            message: request.get_message_body(),
        });
    }
}
```

Then expose and register it:

```rust
pub mod wave;
pub use wave::Wave;

self.register_message("WAVE", Wave);
```

Use `talk.rs`, `move_event.rs`, `get_strip.rs`, and `version_check.rs` as references.

</details>

<details>
<summary>Accept a compatibility packet</summary>

Some old clients send packets that only need to be accepted so the client can continue. Register those with `CompatibilityNoop` instead of writing an empty handler:

```rust
self.register_message("SOME_OLD_HEADER", CompatibilityNoop);
```

Use this when matching historical client behaviour before the feature itself is implemented.

</details>

<details>
<summary>Add an outgoing packet</summary>

Outgoing packets live in `src/messages/outgoing/`.

1. Create a message file, for example `src/messages/outgoing/wave_ok.rs`.
2. Implement `OutgoingMessage`.
3. Use `response.init("HEADER")` for the packet header.
4. Use `append_argument` or `append_new_argument` for packet fields.
5. Export it from `src/messages/outgoing/mod.rs`.
6. Add a unit test that checks the composed packet.

```rust
use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveOk {
    username: String,
}

impl WaveOk {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
        }
    }
}

impl OutgoingMessage for WaveOk {
    fn write(&self, response: &mut NettyResponse) {
        response.init("WAVE_OK");
        response.append_argument(&self.username);
    }
}
```

Handlers can send it immediately:

```rust
context.send(WaveOk::new("alice").compose());
```

For examples, see `hello.rs`, `chat.rs`, `wallet_balance.rs`, and `flat_info.rs`.

</details>

<details>
<summary>Add a command-driven feature</summary>

Handlers should stay small. If a packet needs game state, database access, room state, inventory refreshes, or cross-player network effects, record an `IncomingCommand` and let the runtime plan it.

```text
IncomingEvent -> IncomingCommand -> IncomingExecutionEffect -> runtime/game/DAO/network effects
```

1. Add a variant to `IncomingCommand` in `src/messages/incoming/incoming_command.rs`.
2. Record it from the packet handler with `context.record(...)`.
3. Map it in `IncomingCommandExecutor::plan_one`.
4. Add a matching `IncomingExecutionEffect` if the command needs a new execution path.
5. Implement runtime or game-side handling where similar effects are already processed.
6. Add tests for the handler and command planning.

Use this path for features that need more than simple packet parsing.

</details>

<details>
<summary>Add a chat command</summary>

User-entered commands beginning with `:` are routed from `IncomingCommand::Talk` through `CommandManager`, except whispers. Command code is under `src/game/commands/`.

1. Add a command type under `src/game/commands/types/`.
2. Implement the local command trait/pattern used by existing command files.
3. Register the command in the command manager.
4. Return `CommandEffect` values for alerts, broadcasts, inventory updates, or other outcomes.
5. Add tests for direct command invocation and chat routing if needed.

Use `about_command.rs`, `help_command.rs`, `sit_command.rs`, and `reload_definitions_command.rs` as references.

</details>

<details>
<summary>Add storage-backed behaviour</summary>

Database access is kept behind DAO types under `src/dao/`. Runtime code wires MySQL-backed implementations into the application loop from `main.rs`.

1. Add or extend a DAO trait/model where the domain already owns the concept.
2. Implement the MySQL query plan in the MySQL DAO module.
3. Keep packet parsing out of the DAO; pass typed values from command/effect planning.
4. Add tests around query mapping and around the game/runtime behaviour that consumes it.
5. Load startup data in `main.rs` only when it is truly needed before accepting clients.

</details>

<details>
<summary>Add room or game behaviour</summary>

Room and gameplay logic should generally live in `src/game/`, not in packet handlers. Handlers translate protocol payloads into commands; game modules decide what those commands mean.

Good extension points include:

- `src/game/room/` for room entry, users, items, schedulers, walking, and effects.
- `src/game/item/` for item definitions, placement, movement, and interaction.
- `src/game/inventory/` for strip/inventory behaviour.
- `src/game/catalogue/` for purchase and catalogue plans.
- `src/game/messenger/` for buddy and messaging behaviour.
- `src/game/moderation/` for call-for-help style behaviour.

Follow the existing plan/executor style: derive a small plan from current state, execute the plan in a focused executor, and test both.

</details>

<details>
<summary>Add a public room port</summary>

Public room ports are loaded from room data and combined with the configured main/private ports during bootstrap.

1. Add the room data to the database.
2. Ensure the room has the expected public room type and port in the seeded data.
3. Keep `server.port` and `server.private.port` reserved for the hotel view and private rooms.
4. Restart the server so bootstrap rebuilds the listen plan.

The runtime listens on all configured ports and tags connections by the port they arrived on.

</details>

<details>
<summary>Add connection or packet logging</summary>

Connection and packet logging is controlled by `roseau.properties`.

1. Set `log.connections=true` to log channel open/close activity.
2. Set `log.packets=true` to log received and sent packet lines.
3. Keep packet logging disabled when testing noisy client flows unless you need protocol traces.
4. If logging behaviour changes, check `src/server/server_connection_handler_tests.rs` and `src/server/player_network_effect_executor_tests.rs`.

</details>

<details>
<summary>Add a configuration setting</summary>

Use this when a feature should be configurable through `roseau.properties` or `habbohotel.properties`.

1. Add the key to the appropriate sample config file.
2. Parse it through the existing config/bootstrap path rather than reading files directly from feature code.
3. Store the typed value in the runtime/game settings object that already owns nearby settings.
4. Add tests for the default value, valid parsing, and invalid parsing if the value has constraints.

Server, database, and logging settings belong in `roseau.properties`. Gameplay defaults belong in `habbohotel.properties`.

</details>

<details>
<summary>Add a scheduled game action</summary>

Use scheduled actions for behaviour that should run during ticks instead of directly inside a packet handler.

1. Look under `src/game/lifecycle/` and existing scheduler modules for the closest pattern.
2. Represent the action as a plan or task first.
3. Convert the plan into explicit runtime effects.
4. Keep socket writes as network effects, not direct writes from the scheduler.
5. Add tests for the plan and for the executor that applies it.

This is the right path for periodic credits, AFK handling, room status updates, and delayed room/item effects.

</details>

<details>
<summary>Add an item interaction</summary>

Item interaction logic lives under `src/game/item/interactors/`.

1. Add an interactor for the new behaviour.
2. Return item, room, runtime, or network effects instead of mutating unrelated state directly.
3. Register the interactor wherever item behaviours are selected.
4. Add focused tests beside the interactor and effect executor tests if new effects are introduced.

Use the bed, chair, pool, and teleporter interactors as examples.

</details>

<details>
<summary>Add tests for a new expansion</summary>

A good feature usually has more than one small test:

1. Handler test: packet input becomes `IncomingContext` commands or responses.
2. Planning test: `IncomingCommand` becomes the expected `IncomingExecutionEffect`.
3. Outgoing test: composed packet text matches the V1 client format.
4. Runtime/game test: state changes happen in the right manager or executor.
5. Regression test: unsupported input fails quietly when matching legacy behaviour.

Run targeted tests while iterating:

```sh
cargo test talk
cargo test incoming_command_executor
```

Run the full suite before handing off:

```sh
cargo test
```

</details>

## Testing

Run all tests:

```sh
cargo test
```

Run one module or test name:

```sh
cargo test talk
cargo test incoming_command_executor
```

Recommended coverage for expansions:

- one test for packet parsing and `IncomingContext` changes,
- one test for command-to-effect planning,
- one test for outgoing packet composition,
- one runtime/game test when stateful behaviour changes.

## Credits

Roseau-rs was originally ported from [Quackster/Roseau](https://github.com/Quackster/Roseau). The project exists to preserve and experiment with the Habbo Hotel V1 client/server protocol and the 2001-era hotel behaviour in a modern Rust codebase.

## License

GPL-3.0-or-later. See `LICENCE.txt`.
