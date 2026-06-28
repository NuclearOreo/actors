# actors

A small Rust project that implements the **actor pattern** on top of [Tokio](https://tokio.rs/), following Alice Ryhl's article [*Actors with Tokio*](https://ryhl.io/blog/actors-with-tokio/).

An actor is an independently spawned async task that owns some state and communicates with the rest of the program **only by passing messages**. Nothing else touches the actor's state directly, which sidesteps locks and shared-mutability headaches.

## The pattern

The implementation is split into two halves, exactly as described in the blog post:

| Half | Role |
| --- | --- |
| **The actor** (`Actor`) | The spawned task. Owns the state and the receiving end of the channel; loops over incoming messages and handles them one at a time. |
| **The handle** (`ActorHandle`) | The public-facing struct everyone else holds. Owns the sending end of the channel and exposes ergonomic `async` methods. It's `Clone`, so many callers can talk to the same actor. |

Communication uses two kinds of Tokio channels:

- **`mpsc`** (multi-producer, single-consumer) — carries messages from any number of handles to the one actor task. Bounded (capacity 8) so the queue can't grow without limit.
- **`oneshot`** — a single-use reply channel. For request/response, the caller creates a `oneshot` pair, ships the `Sender` inside the message, and `await`s the `Receiver`.

## Layout

```
src/
├── main.rs            # demo: builds a handle, calls its async methods
└── actor/
    ├── mod.rs         # module wiring; re-exports ActorHandle
    ├── msg.rs         # Msg enum — every message the actor accepts
    ├── actor.rs       # Actor struct + run loop + message handling
    └── handler.rs     # ActorHandle — spawns the actor, sends messages
```

### `msg.rs` — the message enum

Each variant carries the data for that request plus a `oneshot::Sender<T>` to send the reply back on.

```rust
pub enum Msg {
    GetUniqueId { respond_to: oneshot::Sender<i32> },
    TrimText { text: String, respont_to: oneshot::Sender<String> },
}
```

### `actor.rs` — the actor

Holds the `mpsc::Receiver` and state (`next_id`). The `run` loop pulls messages until every sender is dropped (at which point `recv()` returns `None` and the actor shuts down cleanly).

```rust
pub async fn run(&mut self) {
    while let Some(msg) = self.receiver.recv().await {
        self.handle_msg(msg);
    }
}
```

### `handler.rs` — the handle

`ActorHandle::new()` creates the channel, builds the `Actor`, and **spawns it with `tokio::spawn`** — spawning lives in the constructor, not inside actor methods, to avoid lifetime trouble. Each public method builds a `Msg`, sends it, and awaits the `oneshot` reply.

```rust
pub async fn get_unique_id(&self) -> i32 {
    let (send, recv) = oneshot::channel();
    let msg = Msg::GetUniqueId { respond_to: send };
    let _ = self.sender.send(msg).await;
    recv.await.expect("Actor task has been killed")
}
```

## Running

```sh
cargo run
```

Expected output:

```
1
"Hello"
```

(`get_unique_id` returns the first id; `trim_text` trims `"Hello     "`.)

## Design notes from the blog post

- **Spawn in the handle's constructor**, not in actor methods.
- **Separate the actor struct from the handle** to keep ownership boundaries clean.
- **Shutdown is automatic**: when all handles (and thus all senders) drop, the run loop ends.
- **Avoid cycles of bounded channels** — two actors that each block waiting on the other's full channel will deadlock.

## Credits

Pattern and code structure from [*Actors with Tokio*](https://ryhl.io/blog/actors-with-tokio/) by Alice Ryhl.
