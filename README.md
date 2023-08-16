# Chapter 12 Raycast

To shoot something, we need to use Raycast.

Now you can see the different if you shoot to the empty sky and shoot to the ground.

We first create the shooting logic in `client.rs` with `game_time()` as we don't want the shoot to be true all the time.

```rust
let mut shoot = false;
if input.mouse_buttons.contains(&MouseButton::Left) {
    if is_shooting {
        if game_time() - last_shot > Duration::from_millis(200) {
            shoot = true;
            last_shot = game_time();
        }
    } else {
        shoot = true;
        is_shooting = true;
        last_shot = game_time();
    }
} else {
    is_shooting = false;
}

```

Then we create the raycast logic in `client.rs` with `screen_position_to_world_ray()`.

```rust
let player_id = player::get_local();
let cam = entity::get_component(player_id, player_cam_ref());
if cam.is_none() {
    return;
}

let cam = cam.unwrap();
let window_size =
    entity::get_component(entity::resources(), window_logical_size()).unwrap();
let ray = camera::screen_position_to_world_ray(
    cam,
    vec2(window_size.x as f32 / 2., window_size.y as f32 / 2.),
);
```

In the `server.rs`, we create the raycast logic with `raycast_first()`.

> There is also a `raycast()` if you want to get all the hits.

```rust
if msg.shoot {
    println!("shoot");
    let result = physics::raycast_first(msg.ray_origin, msg.ray_dir);
    if let Some(hit) = result {
        println!("hit: {:?}", hit);
    }
}
```
