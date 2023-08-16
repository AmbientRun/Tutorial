# Chapter 6 Move character with physisc

In previous chapter we set the translation directly with a calculated new position.

Although this is useful in some cases, e.g. respawn a player to home location, it's not idea for basic movement.

This is because if the scale of translation change is large, the character will look like teleporting.

In this chapter we will use physics to move the character.

To do so, we need to create some components on the character:

```rust
spawn_query(is_player()).bind(|r| {
    for (e, _) in r {
        entity::add_components(
            e,
            Entity::new()
                .with(translation(), vec3(0.0, 0.0, 3.0))
                .with(cube(), ())
                .with(color(), random::<Vec3>().extend(0.8))
                .with(player_direction(), Vec2::ZERO) // a customized component
                .with(physics_controlled(), ()) // the physics control component
                .with(character_controller_height(), 2.0) // required for physics control to work
                .with(character_controller_radius(), 0.5), // required for physics control to work
        );
    }
});

```

So, `physics_controlled()`, `character_controller_height()` and `character_controller_radius()` often come together.

We also define a new component `player_direction()` to store the movement input from user. It's writen in the `ambient.toml`:

```toml
[components]
player_movement_direction = { type = "Vec2", name = "Player movement direction", description = "The player's movement direction.", attributes = [
    "Debuggable",
] }
```

Then in the Input message subscription, we update the `player_direction()` component:

```rust
Input::subscribe(|source, msg| {
    let Some(player_id) = source.client_entity_id() else { return; };
    entity::add_component(player_id, player_direction(), msg.direction);
});
```

With `add_component`, the component will be added if it doesn't exist, or updated if it exists.

Finally, we will use `query` to get all entities that contain `player_direction()` component:

```rust
query(player_direction()).each_frame(move |players| {
        for (player_id, direction) in players {
            let speed = 0.1;
            let displace = (direction * speed).extend(-0.1); // extend the Z axis to fake a gravity
            physics::move_character(player_id, displace, 0.01, delta_time());
        }
    });
```
