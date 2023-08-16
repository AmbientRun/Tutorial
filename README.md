# Chapter 4 Messages

In this chapter we will explore how to use the message system to communicate between the client and server.

To define a message, we write in `ambient.toml`:

```toml
[messages.Hello]
name = "Hello"
description = "Sent when a client joins the server, then sent back from the server"
fields = { cube_id = "EntityId" }
```

We should declare using the message in the client and server:

```rust
use embers::tutorial::messages::Hello;
```

Then in the `server.rs`:

```rust
Hello { cube_id: e }.send_client_broadcast_reliable();
```

In `client.rs`, we subsctibe to the message:

```rust
Hello::subscribe(|source, msg| {
    println!("Hello from {:?}. The msg is {:?}", source, msg);
    let c = entity::get_component(msg.cube_id, color()).unwrap();
    println!("The color is {:?}", c);
});
```

With the `source` and `msg` information, we can do more things.

> There are other types of messages sending, see this [example]().

> Read about the difference between `reliable` and `unreliable` [here]().
