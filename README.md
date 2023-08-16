# Chapter 5 Input

We need to define the Input message in order to send from client to the server.

```toml
[messages.Input]
description = "Describes the input state of the player."
[messages.Input.fields]
direction = "Vec2"
mouse_delta = "Vec2"

```

In the server, we attach a cube with random color to the player to show the position of the player.

Then at every frame, we will get the current input state from the client and update the position of the player.
